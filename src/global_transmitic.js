const GREEN = "#66ff6b";
const RED = "#ff4545";

const SUCCESS = GREEN;
const ERROR = RED;

function setMsgBoxSuccess(msg) {
    setMsgBox(msg, SUCCESS);
}

function setMsgBoxError(msg) {
    setMsgBox(msg, ERROR);
}

function setMsgBox(msg, color) {
    document.$("div#msg-box").innerHTML = `
    <div style="display: inline-block; margin-right: auto;">${escapeHTML(msg)}</div>
    <img id="msg-box-close" src="ic_fluent_dismiss_square_24_filled.svg"
        style="margin-left: auto; display: inline-block; vertical-align:middle; padding-right: 40dip;" />
`
    document.$("div#msg-box").style.display = "block";
    document.$("div#msg-box").style["background-color"] = color;

    document.$("img#msg-box-close").onclick = function () {
        document.$("div#msg-box").style.display = "none";
    }
}

function escapeHTML(html) {
    let newHtml = `${html}`;
    newHtml.replaceAll('&', '&amp;').replaceAll('>', '&gt;').replaceAll('<', '&lt;').replaceAll('"', '&quot;').replaceAll("'", '&#039;');
    return newHtml;
}