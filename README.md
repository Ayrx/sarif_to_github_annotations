# sarif_to_github_annotations

This action processes a [SARIF file](https://sarifweb.azurewebsites.net/) into
GitHub annotations.

```yaml
- if: failure()
uses: Ayrx/sarif_to_github_annotations@v0.2.0
with:
  sarif_file: semgrep.sarif
```
