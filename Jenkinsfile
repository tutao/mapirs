pipeline {
	environment {
		VERSION = powershell(
		    returnStdout: true,
		    script: $/(Select-String -Pattern '^version = "(?<v>\d+.\d+.\d+)"' -AllMatches -Path Cargo.toml).Matches[0].Groups[1].value/$
		).trim()
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
			    powershell "cargo test"
			}
		}

		stage('build the dll') {
			steps {
				echo "building mapirs v${VERSION}"
				powershell 'cargo build --release'
				stash includes: "target/release/mapirs.dll", name: 'dll'
			}
		}

		stage('publish a github release') {
			when {
				expression { params.PUBLISH }
			}
			steps {
				unstash 'dll'

				script {
					def filePath = "target/release/mapirs.dll"
					def tag = "mapirs-release-${VERSION}"

					powershell "git tag ${tag}"
					powershell "git push --tags"

					def checksum = powershell(
					    returnStdout: true,
					    script: "(Get-FileHash -Algorithm SHA256 ${WORKSPACE}/${filePath}).Hash.ToLower()"
					)

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
