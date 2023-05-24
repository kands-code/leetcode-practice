# p0014 最长公共前缀

编写一个函数来查找字符串数组中的最长公共前缀

如果不存在公共前缀，返回空字符串 `""`

## 示例 1

```text
输入：strs = ["flower","flow","flight"]
输出："fl"
```

## 示例 1

```text
输入：strs = ["dog","racecar","car"]
输出：""
解释：输入不存在公共前缀。
```

## 提示

- $1 \le \texttt{strs.length} \le 200$
- $0 \le \texttt{strs[i].length} \le 200$
- `strs[i]` 仅由小写英文字母组成

## 解答

由于题目中说 `strs[i]` 仅由小写英文字母组成，所以就无须考虑其他字符或者大小写问题

这里最直接的方法就是扫描，然后不断 `push`，不过效率肯定很差