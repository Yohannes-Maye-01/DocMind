# 🤝 Contributing to DocMind

Thank you for your interest in contributing to DocMind! We welcome contributions from developers, designers, and enthusiasts. This document provides guidelines and instructions for contributing to the project.

---

## 📋 Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Contribution Types](#contribution-types)
5. [Workflow](#workflow)
6. [Coding Standards](#coding-standards)
7. [Testing](#testing)
8. [Commit Guidelines](#commit-guidelines)
9. [Pull Request Process](#pull-request-process)
10. [Reporting Issues](#reporting-issues)
11. [Documentation](#documentation)
12. [Community](#community)

---

## 🤝 Code of Conduct
We are committed to providing a welcoming and inclusive environment for all contributors. Please adhere to the following principles:

- **Respectful Communication**: Treat all community members with respect
- **Inclusive Environment**: Welcome diverse perspectives and backgrounds
- **Professional Behavior**: Keep discussions focused and constructive
- **Zero Tolerance**: We do not tolerate harassment, discrimination, or hostile behavior

**Violations**: Report violations to support@docmind.dev with details and evidence.

---
## 🚀 Getting Started

### Prerequisites

Before contributing, ensure you have the following installed:

- **Node.js** v18+
- **Python** 3.9+
- **Rust** 1.70+
- **Go** 1.21+
- **Docker** & **Docker Compose** (optional but recommended)
- **Git**

### Fork & Clone

```bash
# 1. Fork the repository on GitHub
# Visit: https://github.com/Yohannes-Maye-01/DocMind

# 2. Clone your fork
git clone https://github.com/YOUR-USERNAME/DocMind.git
cd DocMind

# 3. Add upstream remote
git remote add upstream https://github.com/Yohannes-Maye-01/DocMind.git

# 4. Verify remotes
git remote -v
```

---

## 🛠️ Development Setup

### Complete Setup

```bash
# Install Node.js dependencies
npm install

# Install Python dependencies
pip install -r requirements.txt

# Build Rust engine
cd search-engine
cargo build --release
cd ..

# Download Go modules
cd backend
go mod download
cd ..
```

### Using Docker

```bash
# Build and start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Verify Installation

```bash
# Test Node.js
npm run dev

# Test Python setup
python -c "import langchain; print('Python OK')"

# Test Rust build
cargo test --release

# Test Go setup
go version
```

---

## 📝 Contribution Types

### 🐛 Bug Fixes
- Focus on fixing reported issues
- Reference the issue number in your PR
- Include test cases for the fix

### ✨ Features
- Check open issues for feature requests
- Discuss major features before implementation
- Follow the existing architecture patterns

### 📖 Documentation
- Improve README, guides, and API docs
- Fix typos and clarify explanations
- Add examples and tutorials

### 🧪 Tests
- Increase code coverage
- Add unit, integration, and e2e tests
- Improve test documentation

### ♻️ Refactoring
- Improve code quality
- Optimize performance
- Reduce technical debt

### 🌐 Localization
- Add language support
- Translate documentation
- Support international users

---

## 🔄 Workflow

### 1. Create a Feature Branch

```bash
# Update main branch
git fetch upstream
git checkout main
git reset --hard upstream/main

# Create feature branch
# Format: feature/{feature-name}, bugfix/{bug-name}, docs/{doc-name}
git checkout -b feature/amazing-feature
```

### 2. Make Changes

```bash
# Edit files in your preferred editor
# Follow the coding standards (see below)

# Check your changes
git status
git diff
```

### 3. Keep Branch Updated

```bash
# Fetch latest changes from upstream
git fetch upstream

# Rebase on latest main
git rebase upstream/main

# If conflicts occur, resolve them and continue
git rebase --continue
```

### 4. Push Changes

```bash
# Push to your fork
git push origin feature/amazing-feature
```

---

## 📐 Coding Standards

### General Guidelines

- **Keep It Simple**: Write clear, readable code
- **DRY Principle**: Don't Repeat Yourself
- **Comments**: Add comments for complex logic
- **Performance**: Optimize for speed and memory

### Language-Specific Standards

#### TypeScript/JavaScript
```typescript
// ✅ Good: Clear naming, proper types
interface DocumentMetadata {
  id: string;
  title: string;
  createdAt: Date;
}

async function searchDocuments(query: string): Promise<Document[]> {
  // Implementation
}

// ❌ Bad: Unclear naming, missing types
async function sd(q) {
  // Implementation
}
```

**Rules:**
- Use `const` by default, `let` when needed
- Use type annotations for function parameters
- Avoid `any` types
- Use kebab-case for file names

#### Python
```python
# ✅ Good: Type hints, docstrings
from typing import List

def process_documents(documents: List[str]) -> dict:
    """
    Process multiple documents and return metadata.
    
    Args:
        documents: List of document paths
        
    Returns:
        Dictionary with processing results
    """
    # Implementation
    pass

# ❌ Bad: No type hints or docstrings
def process_docs(docs):
    # Implementation
    pass
```

**Rules:**
- Use type hints for all functions
- Write docstrings (Google style)
- Follow PEP 8
- Use descriptive variable names

#### Rust
```rust
// ✅ Good: Clear error handling, documentation
/// Searches for documents matching the query.
///
/// # Arguments
/// * `query` - The search query string
///
/// # Returns
/// Result with vector of matching documents
pub fn search(query: &str) -> Result<Vec<Document>, SearchError> {
    // Implementation
}

// ❌ Bad: No documentation, unclear error handling
pub fn search(q: &str) -> Vec<Document> {
    // Implementation
}
```

**Rules:**
- Write doc comments for public items
- Handle errors properly
- Use meaningful variable names
- Avoid unwrap() in production code

#### Go
```go
// ✅ Good: Clear naming, proper error handling
func SearchDocuments(query string) ([]Document, error) {
    if query == "" {
        return nil, fmt.Errorf("query cannot be empty")
    }
    // Implementation
}

// ❌ Bad: Missing error handling
func SearchDocuments(query string) []Document {
    // Implementation
}
```

**Rules:**
- Use CamelCase for exported functions
- Always handle and return errors
- Write comments for exported functions
- Keep functions focused and small

---

## 🧪 Testing

### Writing Tests

#### TypeScript/Jest
```typescript
describe('SearchService', () => {
  it('should find documents matching query', async () => {
    const results = await searchDocuments('test');
    expect(results.length).toBeGreaterThan(0);
  });

  it('should return empty array for non-existent query', async () => {
    const results = await searchDocuments('xyzabc123');
    expect(results).toEqual([]);
  });
});
```

#### Python/pytest
```python
def test_search_documents():
    """Test document search functionality."""
    results = search_documents("test")
    assert len(results) > 0

def test_search_non_existent():
    """Test search with non-existent query."""
    results = search_documents("xyzabc123")
    assert results == []
```

#### Rust
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_documents() {
        let results = search("test").unwrap();
        assert!(!results.is_empty());
    }
}
```

#### Go
```go
func TestSearchDocuments(t *testing.T) {
    results, err := SearchDocuments("test")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(results) == 0 {
        t.Error("expected results, got none")
    }
}
```

### Running Tests

```bash
# TypeScript/JavaScript
npm test

# Python
pytest tests/

# Rust
cargo test --release

# Go
go test ./...
```

### Coverage Requirements

- Aim for **80%+ code coverage**
- New features should include tests
- Bug fixes should include regression tests

---

## 💬 Commit Guidelines

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only
- **style**: Changes that don't affect code logic
- **refactor**: Code refactoring
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **ci**: CI/CD configuration

### Examples

```bash
# Feature commit
git commit -m "feat(search): add semantic search capability"

# Bug fix with reference
git commit -m "fix(api): resolve timeout in document upload
Fixes #123"

# Documentation update
git commit -m "docs: add setup guide for Rust engine"
```

### Best Practices

- Write in **present tense** ("add" not "added")
- Keep subject **under 50 characters**
- Reference issues and PRs: "Fixes #123", "Relates to #456"
- Use the body to explain **why**, not what
- Commit related changes together
- Avoid mixing refactoring with features

---

## 🔀 Pull Request Process

### Before Creating a PR

1. **Update your branch**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run tests locally**
   ```bash
   npm test
   pytest tests/
   cargo test --release
   go test ./...
   ```

3. **Check code quality**
   ```bash
   npm run lint
   cargo clippy
   go vet ./...
   ```

### Creating a PR

1. **Push your branch**
   ```bash
   git push origin feature/amazing-feature
   ```

2. **Open PR on GitHub**
   - Go to https://github.com/Yohannes-Maye-01/DocMind
   - Click "New Pull Request"
   - Select your branch and fill out the template

### PR Template

```markdown
## Description
Brief description of changes

## Type
- [ ] Bug fix
- [ ] Feature
- [ ] Documentation
- [ ] Performance improvement

## Related Issues
Fixes #123

## Testing
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] No breaking changes
- [ ] Tested on relevant platforms
```

### PR Guidelines

- **One concern per PR**: Keep PRs focused and reviewable
- **Descriptive title**: Use the format `type: description`
- **Clear description**: Explain what and why
- **Link issues**: Reference related issues
- **Include tests**: Add tests for new code
- **Update docs**: Keep documentation in sync

### Review Process

1. **Automated checks** will run (tests, linting)
2. **Maintainers will review** your code
3. **Respond to feedback** promptly
4. **Update PR** based on review comments
5. **Merge** once approved

---

## 🐛 Reporting Issues

### Security Issues

**Do not open public issues for security vulnerabilities.**

- Email: security@docmind.dev
- Include: Version, steps to reproduce, impact assessment
- Allow time for a fix before disclosure

### Bug Reports

Use the bug report template:

```markdown
## Description
Clear description of the bug

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: 
- Browser: 
- Version: 

## Logs/Screenshots
Relevant logs or screenshots
```

### Feature Requests

```markdown
## Description
Clear description of the feature

## Use Case
Why is this needed?

## Proposed Solution
How should it work?

## Alternatives
Other approaches considered
```

---

## 📚 Documentation

### Contributing to Docs

1. **Documentation Structure**
   ```
   docs/
   ├── README.md           # Overview
   ├── setup.md            # Setup instructions
   ├── architecture.md     # Architecture details
   ├── api.md              # API documentation
   ├── contributing.md     # This file
   └── guides/             # Additional guides
   ```

2. **Writing Guidelines**
   - Use clear, simple language
   - Add code examples
   - Include screenshots when helpful
   - Keep sections short and focused
   - Link to related documents

3. **Format**
   - Use Markdown
   - Include table of contents
   - Add code blocks with language specified
   - Use headings hierarchically

---

## 🌍 Community

### Communication Channels

- **GitHub Discussions**: General questions and discussions
- **GitHub Issues**: Bug reports and feature requests
- **Email**: support@docmind.dev
- **Discussions Board**: Community support

### Getting Help

1. Check existing issues and discussions
2. Read documentation and guides
3. Ask in GitHub Discussions
4. Email for security concerns

### Recognition

Contributors will be:
- Added to CONTRIBUTORS.md
- Mentioned in release notes
- Recognized in project README

---

## 📊 Development Tips

### Useful Commands

```bash
# Format code
npm run format

# Lint code
npm run lint

# Type check TypeScript
npm run type-check

# Build everything
npm run build

# Run in development mode
npm run dev

# Generate documentation
npm run docs
```

### Debugging

```bash
# Node.js debugging
node --inspect-brk node_modules/.bin/jest

# Python debugging
python -m pdb your_script.py

# Rust debugging
rust-lldb ./target/debug/docmind

# Go debugging
dlv debug ./cmd/main.go
```

---

## 🎯 Good First Issues

New to the project? Look for issues labeled:
- `good-first-issue`
- `help-wanted`
- `documentation`
- `beginner-friendly`

---

## ❓ FAQ

**Q: How long does PR review take?**
A: Usually 2-5 business days, depending on complexity.

**Q: Can I work on an issue that's already assigned?**
A: Ask in the issue comments first.

**Q: Do I need to sign a CLA?**
A: No, but we do require commits to be signed.

**Q: How do I update my PR after review?**
A: Make new commits, push to your branch, and the PR updates automatically.

---

## 🙏 Thank You

Your contributions make DocMind better! We appreciate your effort and look forward to working with you.

**Happy Contributing! 🚀**
