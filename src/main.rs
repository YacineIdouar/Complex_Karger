use rand::Rng;

fn dot (matrice : &Vec <Vec<i32>>) ->String {
    let mut graph = String::new();
    graph = format! ("{}{}",graph,String::from("graph {"));
    for i in 0..matrice.len() {
        for j in (i+1)..matrice.len(){
            for _ in 0..matrice[i][j]{
               graph = format!("{}{}{}{}{}",graph ,i.to_string(),String::from(" -- "), j.to_string(),String::from(" \n"));
            }
        }
    }
    graph = format! ("{}{}",graph,String::from("}"));
    graph
}

// Fin initialisation !!
fn init_matrice(matrice : &mut Vec <Vec<i32>>, liste_arete : &mut Vec<(i32,i32)>, n : usize) -> () {
    for i in 0..n {
        for j in (i+1)..n {
            if i==j {
                matrice[i][j] = 0;
            }else {
                let val = rand::thread_rng().gen_range(0..2);
                matrice[i][j] = val;
                matrice[j][i] = val;
                if val == 1 {
                    liste_arete.push((i.try_into().unwrap(),j.try_into().unwrap()));
                }
            }
        }
    }
}

// Fonction de contraction  sur la matrice d'adjacence 
fn contraction (matrice : &mut Vec <Vec<i32>>, liste_arete : &mut Vec<(i32,i32)>, sommet1 : usize, sommet2 : usize) -> (){
   // Retrait des arete de la liste des aretes 
        liste_arete.retain(|(x,y)| (!(*x==sommet2.try_into().unwrap() || *y == sommet2.try_into().unwrap())));
   
   // Mise a jour de la matrice
    for i in 0..matrice.len(){
        // Mise ajour de la ligne
        if i != sommet1{
        if matrice[sommet2][i] != -1 {
        matrice[sommet1][i]  += matrice[sommet2][i];
        matrice[i][sommet1] = matrice[sommet1][i];
        }

        for _j in 0..matrice[sommet2][i] {
            liste_arete.push((sommet1.try_into().unwrap(),i.try_into().unwrap()));
        }
    }
        // Supression du sommet en mettant toute ses valeurs a -1
        matrice[sommet2][i] = -1;
        matrice[i][sommet2] = -1;
    }
    
}


fn main() {
    let n : usize = 15;
    let mut matrice = vec![vec![0;n];n];
    let mut liste_arete : Vec<(i32,i32)> = Vec::new();
    init_matrice(&mut matrice, &mut liste_arete, n);

    for i in 0..n{
        println!("{:?}",matrice[i]);
    }
    println!("{:?}",liste_arete);

    println!("Le graphe dot avant : \n {}",dot(&matrice));
    contraction(&mut matrice, &mut liste_arete, 0, 2);

     println!("Après contraction !");

    for i in 0..n{
        println!("{:?}",matrice[i]);
    }
    println!("{:?}",liste_arete);

    println!("Le graphe dot après : \n{}",dot(&matrice));

    contraction(&mut matrice, &mut liste_arete, 0, 3);

    println!("Après contraction !");

   for i in 0..n{
       println!("{:?}",matrice[i]);
   }
   println!("{:?}",liste_arete);

   println!("Le graphe dot après : \n{}",dot(&matrice));
}
