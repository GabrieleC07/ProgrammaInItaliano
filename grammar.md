$$
\begin{align}

    [\text{Prog}] &\to [\text{stmt}]^+
    \\
    [\text{stmt}] &\to 
        \begin{cases}
            \text{return([\text{expr}])}
            \\
            \text{var} \space ident \space \text{= [expr]} 
        \end{cases}
    \\
    [\text{expr}] &\to 
        \begin{cases}
                \text{int\_lit}
                \\
                \text{ident}
        \end{cases}

\end{align}

$$