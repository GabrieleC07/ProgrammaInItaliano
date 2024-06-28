$$
\begin{align}

    [\text{Prog}] &\to [\text{stmt}]^+
    \\
    [\text{stmt}] &\to 
        \begin{cases}
            return([\text{expr}])
            \\
            \text{var} \space ident \space \text{= [expr]} 
        \end{cases}
    \\
    [\text{expr}] &\to \text{int\_lit}

\end{align}

$$