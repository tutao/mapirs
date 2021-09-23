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
			    pwsh 'cargo clean'
 			    pwsh 'cargo test --target "x86_64-pc-windows-msvc"'
			}
		}

		stage('build the dll') {
			steps {
				echo "building mapirs v${VERSION}"
				pwsh 'cargo clean'
				pwsh 'cargo build --release --target "x86_64-pc-windows-msvc"'
				stash includes: "target/release/x86_64-pc-windows-msvc/mapirs.dll", name: 'dll'
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
                    $checksum = (Get-FileHash -Algorithm SHA256 -Path $Env:RELEASE_ASSET_PATH).Hash.ToLower()
                    $url = "https://api.github.com/repos/tutao/mapirs/releases"

                    $body_data = @{
                        'tag_name' = $Env:RELEASE_TAG
                        'name' = "mapirs v" + $Env:VERSION
                        'body' = "sha 256 checksum: " + $checksum
                    }

                    $body = $body_data | ConvertTo-Json

                    $headers = @{
                        'Accept' = 'application/vnd.github.v3+json'
                        'Authorization' = "token $Env:GITHUB_TOKEN"
                    }

                    $resp = Invoke-RestMethod -Method 'POST' -Uri $url -Body $body -Headers $headers

                    $asset_url = "https://uploads.github.com/repos/tutao/mapirs/releases/" + $resp.id + "/assets?name=mapirs.dll"
                    $headers["Content-Type"] = "application/octet-stream"
                    $asset_resp = Invoke-RestMethod -Method 'POST' -Uri $asset_url -InFile $Env:RELEASE_ASSET_PATH -Headers $headers
                '''
			}
		}
	}
}
