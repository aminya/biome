use crate::snap_test::{SnapshotPayload, assert_file_contents};
use crate::{FORMATTED, assert_cli_snapshot, run_cli};
use biome_console::BufferConsole;
use biome_fs::{FileSystemExt, MemoryFileSystem};
use bpaf::Args;
use camino::Utf8Path;

const SUPPRESS_BEFORE: &str = "(1 >= -0)";
const SUPPRESS_AFTER: &str =
    "// biome-ignore lint/suspicious/noCompareNegZero: ignored using `--suppress`\n(1 >= -0)";

const SUPPRESS_WITH_REASON: &str =
    "// biome-ignore lint/suspicious/noCompareNegZero: We love Biome\n(1 >= -0)";

#[test]
fn ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let (_, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--suppress", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
}

#[test]
fn err_when_both_write_and_suppress_are_passed() {
    let fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--suppress", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "err_when_both_write_and_suppress_are_passed",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--suppress", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_multiple_ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(
        file_path.into(),
        [SUPPRESS_BEFORE, SUPPRESS_BEFORE].join("\n").as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--suppress", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, [SUPPRESS_AFTER, SUPPRESS_AFTER].join("\n"));

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_multiple_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_only_ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--suppress",
                "--only=lint/suspicious/noCompareNegZero",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_AFTER);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_only_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppress_skip_ok() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--suppress",
                "--skip=lint/suspicious/noCompareNegZero",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_skip_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn err_when_only_reason() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--reason", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "err_when_only_reason",
        fs,
        console,
        result,
    ));
}

#[test]
fn custom_explanation_with_reason() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("fix.js");
    fs.insert(file_path.into(), SUPPRESS_BEFORE.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--suppress",
                "--reason=We love Biome",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, SUPPRESS_WITH_REASON);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "custom_explanation_with_reason",
        fs,
        console,
        result,
    ));
}

#[test]
fn unused_suppression_after_top_level() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"/**
* biome-ignore-all lint/style/useConst: reason
*/


let foo = 2;
/**
* biome-ignore lint/style/useConst: reason
*/
let bar = 33;",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unused_suppression_after_top_level",
        fs,
        console,
        result,
    ));
}

#[test]
fn unsafe_write_removes_only_unused_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"// biome-ignore lint/suspicious/noDebugger: unused
function read(value) {
  return value;
}
read(1);
// biome-ignore lint/suspicious/noDebugger: used
debugger;
",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(
        &fs,
        file_path,
        "\nfunction read(value) {\n  return value;\n}\nread(1);\n// biome-ignore lint/suspicious/noDebugger: used\ndebugger;\n",
    );
}

#[test]
fn unsafe_check_removes_unused_suppression_for_non_fixable_rule() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"// biome-ignore lint/performance/noAwaitInLoops: unused
const value = 1;
console.log(value);
",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "check",
                "--only=lint/performance/noAwaitInLoops",
                "--write",
                "--unsafe",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, "const value = 1;\nconsole.log(value);\n");
}

#[test]
fn unsafe_write_removes_unused_suppression_for_all_analyzer_languages() {
    for (file_path, selector, content, expected) in [
        (
            "file.css",
            "--only=lint/correctness/noUnknownProperty",
            "/* biome-ignore lint/correctness/noUnknownProperty: unused */\na { color: red; }\n",
            "\na { color: red; }\n",
        ),
        (
            "file.html",
            "--only=lint/a11y/noSvgWithoutTitle",
            "<!-- biome-ignore lint/a11y/noSvgWithoutTitle: unused -->\n<div></div>\n",
            "\n<div></div>\n",
        ),
        (
            "file.graphql",
            "--only=lint/suspicious/noEmptySource",
            "# biome-ignore lint/suspicious/noEmptySource: unused\nquery Hello { field }\n",
            "\nquery Hello { field }\n",
        ),
        (
            "file.jsonc",
            "--only=lint/suspicious/noDuplicateObjectKeys",
            "// biome-ignore lint/suspicious/noDuplicateObjectKeys: unused\n{\"key\": 1}\n",
            "\n{\"key\": 1}\n",
        ),
    ] {
        let fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Utf8Path::new(file_path);
        fs.insert(file_path.into(), content.as_bytes());

        let (fs, result) = run_cli(
            fs,
            &mut console,
            Args::from(["lint", selector, "--write", "--unsafe", file_path.as_str()].as_slice()),
        );

        assert!(result.is_ok(), "{file_path}: run_cli returned {result:?}");
        assert_file_contents(&fs, file_path, expected);
    }
}

#[test]
fn unsafe_write_keeps_unused_plugin_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore lint/suspicious/noDebugger lint/plugin/noManualZIndex: keep the plugin suppression\nconst value = 1;\nconsole.log(value);\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn unsafe_write_keeps_known_but_disabled_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore lint/security/noSecrets: keep for the security configuration\nconst value = \"ordinary string\";\nconsole.log(value);\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn unsafe_write_removes_unused_explicitly_enabled_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        Utf8Path::new("biome.json").into(),
        br#"{
  "linter": {
    "rules": {
      "security": {
        "noSecrets": "error"
      }
    }
  }
}"#,
    );

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        "// biome-ignore lint/security/noSecrets: unused\nconst value = \"ordinary string\";\nconsole.log(value);\n".as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(
        &fs,
        file_path,
        "\nconst value = \"ordinary string\";\nconsole.log(value);\n",
    );
}

#[test]
fn unsafe_write_keeps_inactive_category_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore assist: keep for the assist command\nconst value = 1;\nconsole.log(value);\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn unsafe_write_keeps_malformed_known_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore lint/security/noSecrets\nconst value = \"ordinary string\";\nconsole.log(value);\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn unsafe_write_keeps_disabled_suppression_in_mixed_comment() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore lint/suspicious/noDebugger lint/security/noSecrets: keep both\nfunction read(value) {\n  return value;\n}\nread(1);\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn unsafe_write_keeps_invalid_suppression_in_mixed_comment() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore lint/suspicious/noDebugger lint/suspicious/notARealRule: keep the invalid entry\nfunction read(value) {\n  return value;\n}\nread(1);\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn unsafe_write_keeps_suppression_with_invalid_explanation() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore lint/suspicious/noDebugger: <explanation>\nconst value = 1;\nconsole.log(value);\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn unsafe_write_keeps_used_no_secrets_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore lint/security/noSecrets: this is a known false positive\nconst awsApiKey = \"AKIA1234567890EXAMPLE\";\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=lint/security/noSecrets",
                "--write",
                "--unsafe",
                file_path.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn unsafe_write_keeps_used_suppression_in_mixed_comment() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    let content = "// biome-ignore lint/suspicious/noDebugger lint/style/useConst: mixed\nlet value = 1; debugger;\nconsole.log(value);\n";
    fs.insert(file_path.into(), content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--write", "--unsafe", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
    assert_file_contents(&fs, file_path, content);
}

#[test]
fn misplaced_top_level_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"
let foo = 2;
/**
* biome-ignore-all lint/style/useConst: reason
* biome-ignore-all lint/suspicious/noDebugger: reason
*/
debugger
let bar = 33;",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "misplaced_top_level_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn unused_range_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-all lint/suspicious/noDoubleEquals: single rule
a == b;
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
a == b;
a == b;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unused_range_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn syntax_rule_line_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore syntax/correctness/noTypeOnlyImportAttributes: bug
import type { ChalkInstance } from \"chalk\" with { \"resolution-mode\": \"import\" };

function sommething(chalk: ChalkInstance) {
  console.log(chalk.yellow('we do something here'));
}",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    if let Result::Err(e) = &result {
        println!("{e:#?}");
    }

    assert!(result.is_ok(), "run_cli returned {result:#?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "syntax_rule_line_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn syntax_rule_range_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-start syntax/correctness/noTypeOnlyImportAttributes: bug
import type { ChalkInstance } from \"chalk\" with { \"resolution-mode\": \"import\" };
import type { ChalkInstance2 } from \"chalk2\" with { \"resolution-mode\": \"import\" };
// biome-ignore-end syntax/correctness/noTypeOnlyImportAttributes: bug

function sommething(chalk: ChalkInstance) {
  console.log(chalk.yellow('we do something here'));
}",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "syntax_rule_range_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn syntax_rule_range_suppression_category_only() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-start lint: explanation
const foo = 1;
// biome-ignore-end lint: explanation",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "syntax_rule_range_suppression_category_only",
        fs,
        console,
        result,
    ));
}

#[test]
fn syntax_rule_top_suppression() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-all syntax/correctness/noTypeOnlyImportAttributes: bug
import type { ChalkInstance } from \"chalk\" with { \"resolution-mode\": \"import\" };
import type { ChalkInstance2 } from \"chalk2\" with { \"resolution-mode\": \"import\" };

function sommething(chalk: ChalkInstance) {
  console.log(chalk.yellow('we do something here'));
}",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "syntax_rule_top_suppression",
        fs,
        console,
        result,
    ));
}

#[test]
fn err_when_missing_range_end() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"
// biome-ignore-start syntax/correctness/noTypeOnlyImportAttributes: bug
import type { ChalkInstance } from \"chalk\" with { \"resolution-mode\": \"import\" };
import type { ChalkInstance2 } from \"chalk2\" with { \"resolution-mode\": \"import\" };

function sommething(chalk: ChalkInstance) {
  console.log(chalk.yellow('we do something here'));
}",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "err_when_missing_range_end",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_emit_diagnostics_for_incorrect_reason() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Utf8Path::new("file.ts");
    fs.insert(
        file_path.into(),
        *b"// biome-ignore-all lint/style/useConst:
var foo = 2;
// biome-ignore-all lint/style/useConst: <explanation>
var bar = 33;",
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", file_path.as_str()].as_slice()),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_emit_diagnostics_for_incorrect_reason",
        fs,
        console,
        result,
    ));
}
