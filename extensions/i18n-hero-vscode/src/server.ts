import * as vscode from "vscode";
import { LogLevel, LogOutputChannel } from "vscode";

import {
  LanguageClient,
  LanguageClientOptions,
  RevealOutputChannelOn,
  ServerOptions,
} from "vscode-languageclient/node";
let client: LanguageClient;
interface BaseOptions {
  output: vscode.OutputChannel;
}
function makeLogOutputChannel(output: vscode.OutputChannel): LogOutputChannel {
  const noLogLevelChange: vscode.Event<LogLevel> = () => ({
    dispose: () => {},
  });
  return {
    name: output.name,
    append: output.append.bind(output),
    appendLine: output.appendLine.bind(output),
    clear: output.clear.bind(output),
    show: output.show.bind(output),
    hide: output.hide.bind(output),
    dispose: output.dispose.bind(output),
    trace: output.appendLine.bind(output),
    debug: output.appendLine.bind(output),
    info: output.appendLine.bind(output),
    warn: output.appendLine.bind(output),
    error: output.appendLine.bind(output),
    logLevel: LogLevel.Info,
    onDidChangeLogLevel: noLogLevelChange,
    replace: (value: string) => {
      output.clear();
      output.append(value);
    },
  };
}
export function start({ output }: BaseOptions) {
  const serverOptions: ServerOptions = {
    command:
      "/home/daniel/Projects/i18n-hero/i18n-hero-cli/target/debug/i18n-hero-cli",
    args: ["lsp"],
  };
  output.append("i18n-hero lsp loading");

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "*" }],
    traceOutputChannel: makeLogOutputChannel(output),
    revealOutputChannelOn: RevealOutputChannelOn.Info,
    outputChannel: makeLogOutputChannel(output),
  };

  client = new LanguageClient(
    "i18n-hero",
    "i18n-hero LSP",
    serverOptions,
    clientOptions,
  );
  output.append("i18n-hero lsp starting");
  client.start().catch((err) => {
    output.appendLine(
      `client.start() failed: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`,
    );
    vscode.window.showErrorMessage("Failed to start i18n-hero LSP...");
  });
  output.show(true);
  // client.setTrace(Trace.Verbose); // optional: very chatty LSP trace logs
}

export function restart({ output }: BaseOptions) {
  vscode.window.showInformationMessage("Restarting i18n-hero LSP...");
  if (client.state !== 2) {
    output.appendLine("i18n-hero LSP is not running. Starting it now.");
    start({ output });
    return;
  }
  client
    .stop()
    .then(() => {
      start({ output });
      output.appendLine("i18n-hero LSP restarted successfully.");
      vscode.window.showInformationMessage(
        "i18n-hero LSP restarted successfully.",
      );
    })
    .catch((err) => {
      vscode.window.showErrorMessage(
        `Failed to restart i18n-hero LSP: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`,
      );
      output.appendLine(
        `Failed to restart i18n-hero LSP: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`,
      );
    });
}

export function stop(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop().catch((err) => {
    vscode.window.showErrorMessage(
      `Failed to stop i18n-hero LSP: ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`,
    );
  });
}
