 $$

\begin{align}
    [\text{Prog}] &\to [\text{Stmt}]^+
    \\
    \text{Scope} &\to \text{\{}\ [\text{Stmt}]^+ \text{\}}
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
    \text{Equality} &\to \text{[Expr]} == \text{[Expr]}
    \\
    \text{if} &\to 
        \begin{cases}
            \text{true} \to \text{\{}\ [\text{Stmt}]^+ \text{\}}
            \\
            \text{false}
        \end{cases}
    \\
    \text{while} &\to
        \begin{cases}
            \text{true} \to \text{\{}\ [\text{Stmt}]^+ \text{\}}
                \\
            \text{false}
        \end{cases}
\end{align}

$$