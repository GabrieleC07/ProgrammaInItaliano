 $$

\begin{align}
    [\text{Prog}] &\to [\text{Stmt}]^+
    \\
    [\text{Stmt}] &\to 
        \begin{cases}
            \text{return([\text{Expr}])}
            \\
            \text{var} \space ident \space \text{= [Expr]}
            \\
        \end{cases}
    \\
    [\text{Expr}] &\to 
        \begin{cases}
                \text{int\_lit}
                \\
                \text{ident}
                \\
                \text{[MathExpr]}
        \end{cases}
    \\
    \text{[MathExpr]} &\to 
        \begin{cases}
            \text{[Expr] * [Expr] order = 1}
            \\
            \text{[Expr] + [Expr] order = 0}
        \end{cases}
    \\
    \text{Scope} &\to \text{\{}\ [\text{Stmt}]^+ \text{\}}
\end{align}

$$