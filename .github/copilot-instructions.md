# Copilot / MCP usage instructions

This file documents conventions and short instructions for using the available MCP (Multi-Component Process) tools in this repository. It is intended for automated assistants (Copilot style agents) that have access to the project's MCP functions.

Summary of available MCPs and how to use them

- mcp_serena (code / repo symbolic tools)
  - Use `mcp_serena` tools for precise, token-efficient code exploration. For edits, prefer integrated editing tools.
  - Preferred order when interacting with code:
    1. `mcp_serena_get_symbols_overview` to get top-level symbols from a file.
    2. `mcp_serena_find_symbol` to find specific symbols (use `depth` and `include_body` as needed).
    3. `mcp_serena_find_referencing_symbols` to find usages when changing a symbol.
    4. For edits: Use `replace_string_in_file` or `create_file` instead of serena's symbolic edit tools.
  - Do NOT read entire files unless strictly necessary. Use the overview + symbol tools first.
  - Use `mcp_serena_search_for_pattern` for non-code files or when you do not know symbol names.
  - **IMPORTANT**: Do NOT use `mcp_serena_insert_before_symbol`, `mcp_serena_insert_after_symbol`, or `mcp_serena_replace_symbol_body` for edits. Use integrated editing tools (`replace_string_in_file`, `create_file`) instead, but continue using serena for code exploration and finding usages.

- mcp_github (github operations)
  - Always call `mcp_github_github_get_me` / `mcp_github_github_get_teams` if you need to check permissions or context.
  - For pull request reviews: use `pull_request_review_write` with method `create` to start a pending review, add line comments with `mcp_github_github_add_comment_to_pending_review`, then submit the pending review with `pull_request_review_write` method `submit_pending`.
  - For file changes in the remote repo: use `mcp_github_github_create_or_update_file` or `mcp_github_github_push_files` rather than making local-only edits.
  - For listing/creating issues and PRs prefer `mcp_github` search/list endpoints to avoid duplicates.

- mcp_cargo (cargo / rust project operations)
  - **IMPORTANT**: Always use the cargo MCP tools for ALL cargo operations. Do NOT run `cargo` commands directly via `run_in_terminal`.
  - Before running cargo operations, call `mcp_cargo-mcp_set_working_directory` (set working directory) so subsequent cargo operations run in the right folder.
  - Use the specific cargo tools (`mcp_cargo-mcp_cargo_build`, `mcp_cargo-mcp_cargo_test`, `mcp_cargo-mcp_cargo_run`, etc.) for build/test/run operations.
  - When running tests programmatically, prefer `mcp_cargo-mcp_cargo_test` to get machine-readable results.
  - Rationale: Using cargo MCP tools ensures the assistant can capture and report structured results, handle errors properly, and maintain consistent workflows.

General tool-usage conventions and safety

- Tool batch preface: Before executing a batch of tool calls (especially edits), include a one-sentence preamble describing why you're running the tools, what you'll run, and the expected outcome. This helps reviewers and automated logs.

- Todo-tracking: Use the `manage_todo_list` tool to plan and track progress for multi-step tasks. Always write the full todo list (not a partial update) when updating.

- Progress cadence: After 3–5 tool calls or after creating / editing more than ~3 files, provide a short progress update describing what ran, key results, and next steps.

- Safety and non-exfiltration: Do not send secrets or private tokens to external endpoints. Avoid running network requests unless explicitly required and approved.

- Editing rules and apply_patch guidance:
  - Use `apply_patch` for file edits in the repository workspace. Keep patches minimal and preserve coding style.
  - When editing files, include context lines and use the @@ markers when needed so the patch can be applied reliably.
  - After edits, run the appropriate build or test checks and report results.

Best practices for code edits

- Read symbols first: prefer `mcp_serena_get_symbols_overview` and `mcp_serena_find_symbol` instead of reading entire files.
- Use serena for exploration only: Use serena tools to find symbols and understand code structure, but use `replace_string_in_file` or `create_file` for actual edits.
- When renaming or changing public symbols, run `mcp_serena_find_referencing_symbols` to find all call-sites that need updating.
- Add small, focused tests for any new public behavior. Use the project's test harness (e.g., `mcp_cargo-mcp_cargo_test`).

Example workflow for a code change

1. Preface tools: one-sentence summary of intent (why/what/outcome).
2. Plan with `manage_todo_list` (write full list including new tasks). Mark one item as `in-progress`.
3. Use `mcp_serena_get_symbols_overview` and `mcp_serena_find_symbol` to locate targets.
4. Make edits via `replace_string_in_file` or `create_file` (NOT serena's editing tools).
5. Run `mcp_cargo-mcp_cargo_check` or `mcp_cargo-mcp_cargo_build` to validate compilation.
6. Run tests with `mcp_cargo-mcp_cargo_test`.
7. Update `manage_todo_list` to mark completed items and provide a short summary.

File creation and scripting conventions

- **Do NOT create Python scripts**: Use Rust debug executables in the `examples/` folder instead for testing/debugging.
- **Never create files with `cat`**: Use the `create_file` tool for any file creation tasks. The `cat` command should only be used for viewing/piping content in terminals.
- For temporary analysis or debugging: Create Rust examples or use `run_in_terminal` for ad-hoc analysis, but prefer committed Rust code in examples.

Contact / troubleshooting

- If an MCP call fails, capture the error and include the exact tool response when asking for help. If a patch fails to apply, re-run the symbol overview and try a more targeted patch.

This document is intentionally concise. If you need a longer canonical guide for a specific MCP (for example advanced uses of `mcp_serena` symbol editing tools or the full `mcp_github` PR-review flow), add a follow-up file in `.github/` or `docs/` with examples.
