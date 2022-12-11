import { path, dialog, fs } from "@tauri-apps/api";
import { writeBinaryFile, exists } from '@tauri-apps/api/fs';
import html2canvas from "html2canvas";
import * as icons from '@icon-park/svg'
export default function pdfPlugin() {
    // 后续需要解决的问题： 如何转md到pdf
    return {
        actions: [
            {
                title: "download",
                icon: icons.Download({}),
                cheatsheet: "$".concat('test', "$"),
                handler: {
                    type: 'action',
                    click: ({ editor, root }) => {
                        let dom = root.querySelector(".bytemd-preview");
                        console.log(dom, "========editor=======Download")
                        // 用以解决截图不完整的问题
                        dom.style.height = "auto";
                        html2canvas(dom, {useCORS: true})
                            .then(async canvas => {
                                try {
                                    console.log(canvas, "========html2canvas=====")
                                    let dataUrl = canvas.toDataURL('image/jpeg');
                                    let savePath = await saveFile(dataUrl);
                                    dialog.message(`下载成功,文件存储在: ${savePath}`)  
                                } catch(e) {
                                    console.log(e, "======error======")
                                    dialog.message(e, { title: 'Tauri', type: 'error' })  
                                }
                                dom.style.height = "100%";
                            })
                        console.log("Download plugin======",);
                        editor.focus();
                    }
                }
            },
        ]
    };
}

async function saveFile(dataurl) {
    if(!dataurl) return;
    let uint8 = getUint8Arr(dataurl);
    let blobData = new Blob([uint8.u8arr], { type: uint8.mime })
    const basePath = await path.downloadDir();
    let savePath = await dialog.save({
        defaultPath: basePath,
    });
    savePath = `${savePath}.jpeg`
    console.log(savePath, "======savePath========");
    if(await exists(savePath)) return Promise.reject("文件已存在")
    let file = new FileReader();
    file.readAsArrayBuffer(blobData);
    file.onload = function (e) {
        let fileU8A = new Uint8Array(e.target.result);
        writeBinaryFile({ contents: fileU8A, path: savePath });
    };
    return savePath
}

/**
 * 二进制容器
 * @param {String} dataurl
 */
function getUint8Arr(dataurl) {
    // 截取base64的数据内容
    let arr = dataurl.split(','),
        mime = arr[0].match(/:(.*?);/)[1],
        bstr = atob(arr[1]),
        // 获取解码后的二进制数据的长度，用于后面创建二进制数据容器
        n = bstr.length,
        // 创建一个Uint8Array类型的数组以存放二进制数据
        u8arr = new Uint8Array(n)
    // 将二进制数据存入Uint8Array类型的数组中
    while (n--) {
        u8arr[n] = bstr.charCodeAt(n)
    }
    return { u8arr, mime }
}