pipeline {
	environment {
		VERSION = pwsh(
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
			    pwsh "cargo test"
			}
		}

		stage('build the dll') {
			steps {
				echo "building mapirs v${VERSION}"
				pwsh 'cargo build --release'
				stash includes: "target/release/mapirs.dll", name: 'dll'
			}
		}

		stage('publish a github release') {
			when {
				expression { params.PUBLISH }
			}

			environment {
			    GITHUB_TOKEN = credentials('github-access-token')
			    RELEASE_TAG = "mapirs-release-${VERSION}"
			    RELEASE_ASSET_PATH = "target/release/mapirs.dll"
			}

			steps {
				unstash 'dll'
                pwsh '''
                    $Env:GITHUB_TOKEN | Out-File cred.txt
                    $checksum = (Get-FileHash -Algorithm SHA256 -Path $Env:RELEASE_ASSET_PATH).Hash.ToLower()
                    $url = "https://api.github.com/repos/tutao/mapirs/releases"
                    $user = "tutao-jenkins"
                    $secure_token = ConvertTo-SecureString -String $Env:GITHUB_TOKEN -AsPlainText -Force
                    $credential = New-Object System.Management.Automation.PSCredential($user, $secure_token)

                    $body_data = @{
                        tag_name = $Env:RELEASE_TAG
                        name = "mapirs v" + $Env:VERSION
                        body = "sha 256 checksum:" + $checksum
                    }

                    $body = $body_data | ConvertTo-Json

                    $headers = @{
                        'Accept' = 'application/vnd.github.v3+json'
                        'Authorization' = "token $Env:GITHUB_TOKEN"
                    }

                    $resp = Invoke-WebRequest -Method 'GET' -Uri $url -Body $body -Headers $headers -SkipHttpErrorCheck
                    $assets_url = $resp.assets_url
                    echo $resp

                '''
			}
		}
	}
}
