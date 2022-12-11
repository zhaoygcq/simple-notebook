import { getContentApi } from "./api/file";
import { path } from "@tauri-apps/api";
export function throttle(func, delay) {
    let prev = 0;

    return (content) => {
        let now = +new Date();
        if (now - prev >= delay) {
            func.call(null, content);
            prev = now;
        }
    }
}

export function debounce(func, delay) {
    let timer = null;

    return (content) => {
        if (timer) clearTimeout(timer);
        timer = setTimeout(() => {
            func.call(null, content);
        }, delay);
    }
}

/**
    * count: number,
    * file_path: 完全路径,
    * update_time: 修改时间
*/
export function handleFolderRes(res) {
    const MdReg = /\.md$/;
    if (!res.length) {
        return res;
    }
    let result = [];
    for (let val of res) {
        let { count, file_path: filePath, update_time: createTime } = val;
        let title = filePath.split(path.sep).pop();
        if (!MdReg.test(title)) continue;
        title = title.replace(MdReg, "");
        result.push({
            count,
            filePath,
            createTime,
            title
        })
    }

    return result;
}

export async function getSearchRes(text, files) {
    let res = [];

    for (let item of files) {
        try {
            let { filePath, title } = item;
            let tempPath = filePath.split(path.sep).slice(-3);
            let originDesc = tempPath.slice(0, 2).join(path.sep);
            let fileContent = await getContentApi(filePath);
            // 内容匹配
            let match = getMatchPosition(text, fileContent);
            if (match.length > 0) {
                res.push({
                    originDesc: originDesc,
                    origin: tempPath.pop(),
                    originPath: filePath,
                    desc: match.map(({ start, end }) => {
                        return fileContent.slice(start, end);
                    }),
                    position: match
                })
            };
        } catch (e) {
            console.log("error============", e);
        }
    }

    // 
    return res;
}

function getMatchPosition(text, fileContent) {
    let reg = new RegExp(text, 'g');
    let match, res = [];
    while (match = reg.exec(fileContent)) {
        if (match) {
            let start = (match.index - 10) < 0 ? 0 : match.index - 10;
            let end = match.index + text.length + 10 > fileContent.length ? fileContent.length : match.index + text.length + 10;
            res.push({
                start,
                end,
            })
        }
    }

    return res;
}
