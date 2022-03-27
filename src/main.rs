
fn main() {
    
    // Array de 4 elementos enteros positivos de 8 bits.
    let mut ip: [u8; 4]; // Solo deberá leer el primer valor del array para identificar el tipo de IP
    let mut new_mask: [u8; 4];

    let a = [255, 0,   0,   0]
    let b = [255, 255, 0,   0]
    let c = [255, 255, 255, 0]

    let mut subnet: u8;
    let mut hosts: u8;
    
}


/* 1. Leer IP
 * 1.1 Identificar el tipo de IP
 * 1.1 De ahí se podrá saber qué valores son la parte de la NET y cuáles son los NODOS.
 * 2. Leer el número de subredes que se quiere tener
 * 3. Tomar ese valor e intentar acercarse a él usando 2^n, donde n es el número de bits
 * 3.1 Se tomará ese n y ese será el número de bits que se necesitarán
 * 3.2 Empezar a convertir la variable mask según el tipo de IP empezando desde la izquierda usando
 *   n (128, 64, 32, 16, 8, 4, 2, 1) usando 1's y 0's.
 * 255.255.0.0
 *         | |
 *        \   /
 * 0000 0000.0000 0000  
 * 
 *
 * 4. Escribir la máscara que nos dará estos valores indicados
 *
 *
 *
 * -----------------------------------------------------------------------------------------
 * TEST:
 *
 * 165.100.0.0 Tipo B
 * Con un número de subredes útiles necesarias: 1000
 * 2^10 = 1024 subredes
 * Como es un tipo B entonces la mask default será: 255.255.0.0 y se tomarán los últimos dos ceros
 * para comenzar a escribir el NODO.
 * 0.0
 *  ||
 *  \/
 *  1111 1111. 1100 0000
 *  Se usa lo de (128, 64, 32, 16, 8, 4, 2, 1) y se convierte en:
 *  255.192
 */

