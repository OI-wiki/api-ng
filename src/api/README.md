# API Docs

## `preview`

### GET

```
GET /preview?path=%2Fmath%2Fpoly%2Ffwt HTTP/1.1

{
    "html": "<p>（本文转载自 <a href=\"https://zhuanlan.zhihu.com/c_1005817911142838272\">桃酱的算法笔记</a> ，原文戳 <a href=\"https://zhuanlan.zhihu.com/p/41867199\">链接</a> ，已获得作者授权）</p>\n<h2>简介</h2>\n<blockquote>\n<p>沃尔什转换（Walsh Transform）是在频谱分析上作为离散傅立叶变换的替代方案的一种方法。—— <a href=\"https://zh.wikipedia.org/zh-cn/%E6%B2%83%E7%88%BE%E4%BB%80%E8%BD%89%E6%8F%9B\">维基百科</a> </p>\n</blockquote>\n<p>其实这个变换在信号处理中应用很广泛，fft 是 double 类型的，但是 walsh 把信号在不同震荡频率方波下拆解，因此所有的系数都是绝对值大小相同的整数，这使得不需要作浮点数的乘法运算，提高了运算速度。</p>\n",
    "title": "快速沃尔什变换"
}
```

当发生错误时，状态码为 404
