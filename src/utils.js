export function throttle(func, delay) {
    let prev = 0;

    return (content) => {
        let now = +new Date();
        if(now - prev >= delay) {
            func.call(null, content);
            prev = now;
        }
    }
}

export function debounce(func, delay) {
    let timer = null;

    return (content) => {
        if(timer) clearTimeout(timer);
        timer = setTimeout(() => {
            func.call(null, content);
        }, delay);
    }
}