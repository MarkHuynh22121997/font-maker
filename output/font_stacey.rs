
pub const HEIGHT: usize = 7;

pub const SET: [&str; 127] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    
r#"
     
     
     
     
     
     
     "#,
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
r#"
_______
7     7
|  7  |
|  |  |
|  !  |
!_____!
       "#,
r#"
_____
7   7
!_  |
 7  |
 |  |
 !__!
     "#,
r#"
_______
7     7
!___  !
|  ___|
|     7
!_____!
       "#,
r#"
_______
7     7
|___  !
 |__  |
|     |
!_____!
       "#,
r#"
_______
7  7  7
|  !  |
!___  |
   7  |
   !__!
       "#,
r#"
_______
7  ___7
|     7
!___  |
7     |
!_____!
       "#,
r#"
_______
7     7
|  ___!
|     7
|  -  |
!_____!
       "#,
r#"
_______
7     7
!___  |
   7  |
   |  |
   !__!
       "#,
r#"
_______
7  _  7
!_   _!
|     |
|  -  |
!_____!
       "#,
r#"
_______
7     7
|  -  |
!___  |
7     |
!_____!
       "#,
"",
"",
"",
"",
"",
"",
"",
r#"
_______
7  _  7
|  _  |
|  7  |
|  |  |
!__!__!
       "#,
r#"
_______
7  _  7
|   __|
|  _  |
|  7  |
!_____!
       "#,
r#"
_______
7     7
|  ___!
|  7___
|     7
!_____!
       "#,
r#"
_____  
7    \ 
|  7  |
|  |  |
|  !  |
!_____!
       "#,
r#"
_______
7     7
|  ___!
|  __|_
|     7
!_____!
       "#,
r#"
_______
7     7
|  ___!
|  __| 
|  7   
!__!   
       "#,
r#"
_______
7     7
|   __!
|  !  7
|     |
!_____!
       "#,
r#"
_______
7  7  7
|  !  |
|     |
|  7  |
!__!__!
       "#,
r#"
____
7  7
|  |
|  |
|  |
!__!
    "#,
r#"
   ____
   7  7
___|  |
7  !  |
|     |
!_____!
       "#,
r#"
_______
7  7  7
|   __!
|     |
|  7  |
!__!__!
       "#,
r#"
____   
7  7   
|  |   
|  !___
|     7
!_____!
       "#,
r#"
__________
7        7
|  _  _  |
|  7  7  |
|  |  |  |
!__!__!__!
          "#,
r#"
_______
7     7
|  _  |
|  7  |
|  |  |
!__!__!
       "#,
r#"
_______
7     7
|  7  |
|  |  |
|  !  |
!_____!
       "#,
r#"
_______
7     7
|  -  |
|  ___!
|  7   
!__!   
       "#,
r#"
_______  
7     7  
|  7  |  
|  |  |  
|  !  |_ 
!_______!
         "#,
r#"
_______
7  _  7
|    _|
|  _ \ 
|  7  |
!__!__!
       "#,
r#"
_______
7     7
|  ___!
!__   7
7     |
!_____!
       "#,
r#"
________
7      7
!__  __!
  7  7  
  |  |  
  !__!  
        "#,
r#"
_______
7  7  7
|  |  |
|  |  |
|  !  |
!_____!
       "#,
r#"
___ ___
7  V  7
|  |  |
|  !  |
|     |
!_____!
       "#,
r#"
___ __ ___
7  V  V  7
|  |  |  |
|  !  !  |
|        |
!________!
          "#,
r#"
_______
7  7  7
!_   _!
|     |
|  7  |
!__!__!
       "#,
r#"
_______
7  7  7
|  !  |
!_   _!
 7   7 
 !___! 
       "#,
r#"
_______
7     7
!___  |
|   __!
|     7
!_____!
       "#,
"",
"",
"",
"",
"",
"",
r#"
_______
7  _  7
|  _  |
|  7  |
|  |  |
!__!__!
       "#,
r#"
_______
7  _  7
|   __|
|  _  |
|  7  |
!_____!
       "#,
r#"
_______
7     7
|  ___!
|  7___
|     7
!_____!
       "#,
r#"
_____  
7    \ 
|  7  |
|  |  |
|  !  |
!_____!
       "#,
r#"
_______
7     7
|  ___!
|  __|_
|     7
!_____!
       "#,
r#"
_______
7     7
|  ___!
|  __| 
|  7   
!__!   
       "#,
r#"
_______
7     7
|   __!
|  !  7
|     |
!_____!
       "#,
r#"
_______
7  7  7
|  !  |
|     |
|  7  |
!__!__!
       "#,
r#"
____
7  7
|  |
|  |
|  |
!__!
    "#,
r#"
   ____
   7  7
___|  |
7  !  |
|     |
!_____!
       "#,
r#"
_______
7  7  7
|   __!
|     |
|  7  |
!__!__!
       "#,
r#"
____   
7  7   
|  |   
|  !___
|     7
!_____!
       "#,
r#"
__________
7        7
|  _  _  |
|  7  7  |
|  |  |  |
!__!__!__!
          "#,
r#"
_______
7     7
|  _  |
|  7  |
|  |  |
!__!__!
       "#,
r#"
_______
7     7
|  7  |
|  |  |
|  !  |
!_____!
       "#,
r#"
_______
7     7
|  -  |
|  ___!
|  7   
!__!   
       "#,
r#"
_______  
7     7  
|  7  |  
|  |  |  
|  !  |_ 
!_______!
         "#,
r#"
_______
7  _  7
|    _|
|  _ \ 
|  7  |
!__!__!
       "#,
r#"
_______
7     7
|  ___!
!__   7
7     |
!_____!
       "#,
r#"
________
7      7
!__  __!
  7  7  
  |  |  
  !__!  
        "#,
r#"
_______
7  7  7
|  |  |
|  |  |
|  !  |
!_____!
       "#,
r#"
___ ___
7  V  7
|  |  |
|  !  |
|     |
!_____!
       "#,
r#"
___ __ ___
7  V  V  7
|  |  |  |
|  !  !  |
|        |
!________!
          "#,
r#"
_______
7  7  7
!_   _!
|     |
|  7  |
!__!__!
       "#,
r#"
_______
7  7  7
|  !  |
!_   _!
 7   7 
 !___! 
       "#,
r#"
_______
7     7
!___  |
|   __!
|     7
!_____!
       "#,
"",
"",
"",
"",
];    
