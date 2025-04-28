use exercise_3::difference;


fn main() {
    let coor: (f32, f32) = (15.3, 45.76);
    println!("The coordinates are {}, {}", coor.0, coor.1);

    {
    let diff= difference(coor.0, coor.1);
    println!("The difference is {}", diff);
    }

    {
        let arr:[f32;2] = [coor.0, coor.1];
        println!("The coordinates are {}, {}", arr[0], arr[1]);
    }

    {
        let tuple: (&str, i32, f32, bool, char, [i32; 2], (f32, f32), ([i32; 2], [bool; 2])) = (
            "John",
            30,
            15.3,
            true,
            'a',
            [1, 2],
            (15.3, 45.76),
            ([1, 2], [true, false]),
            
        );
    
        println!("The coordinates are {}, {}", tuple.6. 0, tuple.6. 1);
        println!("The coordinates are {}, {}", tuple.7.0[0], tuple.7.0 [1]);
        println!("The coordinates are {}, {}", tuple.7.1[0], tuple.7.1 [1]);
    }

    

}


  

