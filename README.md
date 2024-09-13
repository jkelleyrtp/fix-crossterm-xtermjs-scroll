# fixing ratatui/crossterm scroll with vscode + xterm.js

crossterm has a bug with vscode + xterm.js that causes the scroll to be broken when raw mode is enabled.

Without the fix, the user needs to scroll up until the original buffer "bottoms out" and then crossterm will start receiving scroll events.

However, if we set MouseCapture to true and then write to the buffer, xtermjs apparently "fixes" the scroll issue. However, we don't want to stay with MosueCapture enabled, since we can't copy-paste logs, so we immediately disable MouseCapture once one event has been received.

This resets xtermjs scroll buffer and fixes the issue completely.
