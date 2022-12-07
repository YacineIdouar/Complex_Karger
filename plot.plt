set terminal pdf
set output 'Krager iteratif matrice'

set xlabel 'Nombre Noeuds -log-'
set ylabel "Temps d'execution(micro)-log-"

set logscale x 2
set logscale y 2 

plot "matrice_Krager_iter.txt" using 1:2 title "matrice" with lines