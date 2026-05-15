pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    let mut visible_len = 0;
    let mut in_ansi = false;
    let mut result = String::new();

    for c in s.chars() {
        if c == '\x1B' {
            in_ansi = true;
        }
        
        if in_ansi {
            result.push(c);
            if c == 'm' {
                in_ansi = false;
            }
            continue;
        }

        visible_len += 1;
        if visible_len > max_width {
            result.push_str("...");
            // 补全可能丢失的 ANSI 重置符
            result.push_str("\x1B[0m");
            break;
        }
        result.push(c);
    }
    result
}

// 提取扩写内容的辅助函数
pub fn extract_context<'a>(
    content: &'a [u8],
    match_start: usize,
    match_end: usize,
    ctx_len: usize,
) -> &'a [u8] {
    // 往前找，不跨越换行符，最多 ctx_len
    let mut start = match_start.saturating_sub(ctx_len);
    if let Some(newline_pos) = memchr::memrchr(b'\n', &content[start..match_start]) {
        start += newline_pos + 1; // 从换行符之后开始
    }

    // 往后找，不跨越换行符，最多 ctx_len
    let mut end = (match_end + ctx_len).min(content.len());
    if let Some(newline_pos) = memchr::memchr(b'\n', &content[match_end..end]) {
        end = match_end + newline_pos; // 到换行符之前结束
    }

    &content[start..end]
}

// 获取当前行号的闭包
pub fn count_lines(content: &[u8], up_to: usize) -> usize {
    memchr::memchr_iter(b'\n', &content[..up_to]).count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_context() {
        let content = b"hello world\nthis is a test\nfor lazyfinder";
        
        // 匹配 "is a"
        let start = 17; // "this is a" -> 'i' index is 17
        let end = 21;   // "is a" length is 4

        // 扩写 5 个字符
        let ctx = extract_context(content, start, end, 5);
        assert_eq!(ctx, b"this is a test");

        // 不跨越换行符测试
        let ctx_large = extract_context(content, start, end, 100);
        assert_eq!(ctx_large, b"this is a test"); // 仍然在当前行
        
        // 独占一行的情况
        let content2 = b"line1\nalone\nline3";
        let start2 = 6;
        let end2 = 11;
        let ctx_alone = extract_context(content2, start2, end2, 10);
        assert_eq!(ctx_alone, b"alone"); // 不会扩写到其它行
    }
}
