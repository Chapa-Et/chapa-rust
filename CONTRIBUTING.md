## Contributing to Chapa-rust

Thank you for your interest in contributing to the Chapa-rust SDK. This guide outlines the workflow, standards, and expectations for all contributions to ensure code quality and consistency.

Before starting to work on your contribution consider the following points.
- **Workflow**: Keep `develop` branch synced with upstream before work.
- **Commits**:
  - Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/):
  ```sh
  <type>(optional-scope): short description
  ```
  - Common types: feat, fix, docs, chore
  - Examples:
    - feat(config): add manifest parsing
    - fix(cli): correct sub-command argument order
    - docs: update setup instructions

- **Documentation**:
  - Public items must have Rust doc comments (`///`).
  - Minimal template:
    ```rust
    /// Short summary sentence.
    ///
    /// # Arguments
    /// * `arg` - description
    ///
    /// # Example
    /// ```
    /// let client = Chapa::new("secret");
    /// ```
    pub fn example(arg: Type) -> Result<T, E> { ... }
    ```

- **Quality Checks**
Run the automations inside makefile before pushing or opening a PR, i.e, format(`make fmt`), lint(`make lint`), tests(`make test`), full check(`make check`)

    Note: Clippy is strict (warnings treated as errors).


- **PR Requirements (merge checklist)**:
  - CI passes (GitHub Actions)
  - make fmt passes
  - make lint passes
  - All tests pass
  - New public APIs have doc comments
  