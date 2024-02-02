import "highlight.js/styles/atom-one-dark.css";
import hljs from "highlight.js/lib/common";

hljs.highlightAll();
hljs.registerLanguage("xml", require("highlight.js/lib/languages/solidity"));
