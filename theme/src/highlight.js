import "highlight.js/styles/atom-one-dark.css";
import hljs from "highlight.js/lib/common";
import solidity from "highlightjs-solidity";

hljs.highlightAll();
solidity(hljs);
hljs.initHighlightingOnLoad();
