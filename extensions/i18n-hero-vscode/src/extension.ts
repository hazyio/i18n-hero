// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";
import { restart, start, stop } from "./server";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  const output = vscode.window.createOutputChannel("i18n-hero");
  context.subscriptions.push(output);

  try {
    const result = start({ output });
    Promise.resolve(result).catch((err) =>
      output.appendLine(
        `start(): failed: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`,
      ),
    );
  } catch (err) {
    output.appendLine(
      `start(): threw: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`,
    );
  }

  const disposable = vscode.commands.registerCommand(
    "i18n-hero-vscode.restart",
    () => {
      try {
        const result = restart({ output });
        Promise.resolve(result).catch((err) =>
          output.appendLine(
            `restart(): failed: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`,
          ),
        );
      } catch (err) {
        output.appendLine(
          `restart(): threw: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`,
        );
      }
    },
  );

  context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() {
  stop();
}
