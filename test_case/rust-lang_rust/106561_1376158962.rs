css
.content .docblock .warning {
	border-left: 2px solid var(--warning-border-color);
	padding: 14px;
	position: relative;
	overflow-x: visible;
	margin-bottom: 10px;
}
.content .docblock .warning:last-child {
	margin-bottom: 0;
}
.content .docblock .warning::before {
	color: var(--warning-border-color);
	content: "ⓘ";
	position: absolute;
	left: -25px;
	top: 5px;
	font-weight: bold;
	font-size: 1.25rem;
}
