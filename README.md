# yellow-chameleon — Sharing Content Securely Between GitHub Repositories

yellow-chameleon is a GitHub action designed to synchronize specific content between repositories. This is particularly useful for scenarios where you want to share a portion of a private repository with the public, while keeping the rest secure.

## Fine-grained Control and Data Privacy

yellow-chameleon empowers you to control precisely what content gets copied, ensuring sensitive information remains confidential. It achieves this by anonymizing commit messages and author details to prevent accidental data leaks.

## Setting Up Yellow-chameleon

Here's a step-by-step guide to get started:

1. **Create Source and Destination Repositories:** You'll need two repositories: one acting as the source (private or private) and another as the destination (public or private). Make sure the destination repository has at least one initial commit.
2. **Generate a Personal Access Token (PAT):** Create a PAT for your account with write access to the destination repository.
3. **Add the PAT to the source repository:** Store the PAT value securely as a GitHub Actions secret within the source repository settings.
4. **Create a Workflow:** Within the `.github/workflows` directory of your source repository, define a workflow named "sync.yml" that looks like this:

```md
name: Sync

on:
  push:
    branches: ['main']

  # Allows you to run this workflow manually from the Actions tab
  workflow_dispatch:

jobs:
  sync:
    concurrency:
      group: ${{ github.workflow }}-sync-test
      cancel-in-progress: false

    runs-on: ubuntu-latest

    steps:
      - name: Use yellow-chameleon
        uses: aaronstanek/yellow-chameleon@v0.3
        with:
          source-path: my/source/path
          destination-repository: username/reponame
          destination-pat: ${{ secrets.PAT }}
          destination-pat-username: username
          git-name: MY NAME
          git-email: my-name@example.com
```

The options for the call to yellow-chameleon are as follows:

- `source-path` (Optional): This allows you to specify a specific directory within the source repository to synchronize. By default, the entire root directory will be copied. Using this option ensures only content within the specified path becomes public (if applicable).

- `destination-repository`: This defines the target repository where the content will be synced to.

- `destination-pat`: This references the PAT secret you created earlier, ensuring secure storage for authentication details.

- `destination-pat-username`: This specifies the username associated with the PAT.

- `git-name`: This sets the author name for commits made in the destination repository.

- `git-email`: This defines the author email for commits made in the destination repository.
