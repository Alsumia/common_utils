/*
Copyright © 2020 Thomas Perreira Pico IV

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//public use so that external crates will be able to use contained macros
pub use cpu_time;

/*
    returns a std::time::Duration which contains the CPU time it took for the function to be 
    executed

    invoked using the name of the function you wish to time and an optional parameter list 
    for the function

    ex:
    let _ = cpu_time_func!(foo);
    let _ = cpu_time_func!(foo, 1, "hello", (1+1));
*/
#[macro_export]
macro_rules! cpu_time_func{

    ($func: ident) =>{
        {
            let point = $crate::cpu_time::ThreadTime::now();
            $func();
            point.elapsed()
        }
    };
   
    ($func: ident,  $($y:expr),+) =>{
        {
            let point = $crate::cpu_time::ThreadTime::now();
            $func($($y,)*);
            point.elapsed()
        }
    }
}

#[cfg(test)]
mod cpu_time_func_tests{


    fn foo(){
        //std::thread::sleep(std::time::Duration::from_millis(10));
    }

    #[test]
    fn test_no_parameters(){
        let _ = cpu_time_func!(foo);
    }

    fn bar(_:i32){

    }

    #[test]
    fn test_one_parameter(){
        let _ = cpu_time_func!(bar, 32);
    }

    fn baz(_:i32, _:&str, _:i64){

    }

    #[test]
    fn test_many_parameters(){
        let _ = cpu_time_func!(baz, 1, "hello", 1+1);
    }
}

