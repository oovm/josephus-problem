

# 一圈N个硬币依次扔， 如果是正面留下， 如果是反面拿走， 直到最后一个， 哪个硬币可能留到最后？

# 一圈N个硬币依次扔， 如果是正面留下， 如果是反面拿走， 直到最后一个， 哪个硬币可能留到最后？

这个题等价于，扔 $$N$$ 个硬币，每个硬币一直扔，直到扔出反面为止，找出扔的次数最多的那枚硬币。如果有平局，编号大的获胜。

对于特定一个硬币，扔出 $$k$$ 次的概率为 $$\begin{align} p_k=\frac{1}{2^k} \end{align}$$

扔出多于 $$k$$ 次的概率为 $$\begin{align} q_k=p_k=\frac{1}{2^k} \end{align}$$

少于或等于 $$k$$ 次的概率为 $$\begin{align} r_k=1-\frac{1}{2^k} \end{align}$$

所以，对于第 $$i$$ 枚硬币，如果他要在第 $$k$$ 轮获胜，那么前 $$i-1$$ 个硬币不能出现比 $$k$$ 大的结果，后面的 $$N-i$$ 个硬币不能出现比 $$k-1$$ 大的结果

所以有

$$\begin{align} P_{i,k}&=\left(r_k\right)^{i-1}\cdot(r_{k-1})^{N-i}\cdot p_k  \\ &=\left(\frac{2^k-1}{2^k}\right)^{i-1}\cdot \left(\frac{2^k-2}{2^k}\right)^{N-i}\cdot\frac{1}{2^k} \\ &=\frac{\left(2^k-1\right)^{i-1}\left(2^k-2\right)^{N-i}}{2^{kN}} \end{align} \\$$ 特殊的 $$\begin{align} P_{N,k}&=\left(r_k\right)^{N-1}\cdot p_k  \\ &=\left(\frac{2^k-1}{2^k}\right)^{N-1}\cdot\frac{1}{2^k} \\ &=\frac{\left(2^k-1\right)^{N-1}}{2^{kN}} \end{align} \\$$

所以对第 $$i$$ 个硬币来说，获胜的总概率为

$$\begin{align} Q_i=\sum_{k=1}^{\infty}{P_{i,k}} \end{align}  \\$$

容易看出当 $$i<j$$ 时， $$P_{i, k} \le P_{j,k}$$ 。所以编号越大的硬币，成为最后的硬币的概率越大

