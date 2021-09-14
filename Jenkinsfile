pipeline {
	environment {
		NODE_PATH="/opt/node-v16.3.0-linux-x64/bin"
		VERSION = sh(returnStdout: true, script: "${NODE_PATH}/node -p -e \"require('./package.json').version\" | tr -d \"\n\"")
		PATH="${env.NODE_PATH}:${env.PATH}"
	}

	agent {
		label 'win-full'
	}

	parameters {
		booleanParam(
			name: 'PUBLISH', defaultValue: false,
			description: "publish the built dll to github"
		 )
	}

	stages {
		stage('run tests') {
			steps {
			    bat "cargo test"
			}
		}

		stage('build the dll') {
			steps {
				echo "building mapirs v${VERSION}"
				bat 'cargo build --release'
				stash includes: "target/x86_64-pc-windows-gnu/release/mapirs.dll", name: 'dll'
			}
		}

		stage('publish a github release') {
			when {
				expression { params.PUBLISH }
			}
			steps {
				sh 'npm ci'

				unstash 'dll'

				script {
					def filePath = "build/app-android/tutanota-${VERSION}-release.apk"
					def tag = "tutanota-android-release-${VERSION}"
					def util = load "jenkins-lib/util.groovy"

					sh "git tag ${tag}"
					sh "git push --tags"

					def checksum = sh(returnStdout: true, script: "sha256sum ${WORKSPACE}/${filePath}")

					withCredentials([string(credentialsId: 'github-access-token', variable: 'GITHUB_TOKEN')]) {
						sh """node buildSrc/createGithubReleasePage.js --name '${VERSION} (Android)' \
																	   --milestone '${VERSION}' \
																	   --tag '${tag}' \
																	   --uploadFile '${WORKSPACE}/${filePath}' \
																	   --platform android \
							 										   --apkChecksum ${checksum}"""
					}
				}
			}
		}
	}
}
