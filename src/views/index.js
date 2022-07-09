export default function pdfPlugin() {
    // 后续需要解决的问题： 如何转md到pdf
    return {
        actions: [
            {
                title: "test",
                icon: "test",
                cheatsheet: "$".concat('test', "$"),
                handler: {
                    type: 'action',
                    click: ({editor}) => {
                        let content = editor.getValue();
                        console.log("test plugin======", content);
                        editor.focus();
                    }
                }
            },
        ]
    };
}
