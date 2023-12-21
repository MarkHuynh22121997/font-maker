
pub const HEIGHT: usize = 9;

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
...$
:::$
---$
+++$
***$
===$
###$
@@@$
   $$"#,
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
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
..ee.$
:@@@:$
%%%%-$
+&&&+$
*|||*$
=!!!=$
#:::#$
@...@$
     $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
++++&&&++$
***|||***$
==!!!====$
#:::::::#$
........@$
         $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
++++&&&++$
****|||**$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
.....eeeee...$
::::@@@@@@:::$
---%%%-%%%---$
++&&&++&&&+++$
*|||||||||||*$
!!!!!!!!!!!!=$
#######:::###$
@@@@@@@...@@@$
             $$"#,
r#"
eeeeeeeee.$
@@@@@@@@@:$
%%%-------$
&&&&&&&&++$
|||||||||*$
======!!!=$
:::::::::#$
@.......@@$
          $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%------$
&&&&&&&++$
||||||||*$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
eeeeeeee.$
@@@@@@@@:$
----%%%--$
+++&&&+++$
**|||****$
=!!!=====$
:::######$
...@@@@@@$
         $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
+&&&&&&++$
*||||||**$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&&+$
*|||||||*$
=====!!!=$
::::::::#$
@......@@$
         $$"#,
"",
"",
"",
"",
"",
"",
"",
r#"
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&&+$
||||||||*$
!!!==!!!=$
:::##:::#$
...@@...@$
         $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&++$
||||||||*$
!!!==!!!=$
::::::::#$
.......@@$
         $$"#,
r#"
.eeeee.$
@@@@@@:$
%%%----$
&&&++++$
|||****$
!!!====$
::::::#$
@.....@$
       $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
::::::::#$
.......@@$
         $$"#,
r#"
eeeeee.$
@@@@@@:$
%%%----$
&&&&&++$
|||||**$
!!!====$
::::::#$
......@$
       $$"#,
r#"
eeeeee.$
@@@@@@:$
%%%----$
&&&&&++$
|||||**$
!!!====$
:::####$
...@@@@$
       $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%------$
&&&++++++$
|||*||||*$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
eee..eee.$
@@@::@@@:$
%%%--%%%-$
&&&&&&&&+$
||||||||*$
!!!==!!!=$
:::##:::#$
...@@...@$
         $$"#,
r#"
eee.$
@@@:$
%%%-$
&&&+$
|||*$
!!!=$
:::#$
...@$
    $$"#,
r#"
eeeeeee.$
@@@@@@@:$
----%%%-$
++++&&&+$
****|||*$
!!==!!!=$
:::::::#$
@.....@@$
        $$"#,
r#"
eee..eee.$
@@@::@@@:$
%%%-%%%--$
&&&&&&+++$
||||||***$
!!!=!!!==$
:::##:::#$
...@@...@$
         $$"#,
r#"
eee....$
@@@::::$
%%%----$
&&&++++$
|||****$
!!!====$
::::::#$
......@$
       $$"#,
r#"
eee......eee.$
@@@@::::@@@@:$
%%%%%--%%%%%-$
&&&&&&&&&&&&+$
|||*||||*|||*$
!!!==!!==!!!=$
:::######:::#$
...@@@@@@...@$
             $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
:::##:::#$
...@@...@$
         $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&&+$
|||||||**$
!!!======$
:::######$
...@@@@@@$
         $$"#,
r#"
.eeeeeeee...$
@@@@@@@@@@::$
%%%----%%%--$
&&&++++&&&++$
|||*||*|||**$
!!!==!!!!!==$
::::::::::##$
@......@...@$
            $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&++$
||||||***$
!!!=!!!==$
:::##:::#$
...@@...@$
         $$"#,
r#"
.eeeeee.$
@@@@@@@:$
%%%-----$
&&&&&&++$
*||||||*$
====!!!=$
:::::::#$
......@@$
        $$"#,
r#"
eeeeeeeee.$
@@@@@@@@@:$
---%%%----$
+++&&&++++$
***|||****$
===!!!====$
###:::####$
@@@...@@@@$
          $$"#,
r#"
eee..eee.$
@@@::@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
eee....eee.$
@@@::::@@@:$
%%%----%%%-$
&&&++++&&&+$
*|||**|||**$
==!!!!!!===$
###::::####$
@@@@..@@@@@$
           $$"#,
r#"
eee..eee..eee.$
@@@::@@@::@@@:$
%%%--%%%--%%%-$
&&&++&&&++&&&+$
|||**|||**|||*$
!!!==!!!==!!!=$
:::::::::::::#$
@...........@@$
              $$"#,
r#"
eee..eee.$
@@@::@@@:$
%%%--%%%-$
+&&&&&&++$
*||||||**$
!!!==!!!=$
:::##:::#$
...@@...@$
         $$"#,
r#"
eee...eee.$
@@@:::@@@.$
%%%---%%%-$
+&&&&&&&++$
**|||||***$
===!!!====$
###:::####$
@@@...@@@@$
          $$"#,
r#"
eeeeeeee.$
@@@@@@@@:$
----%%%--$
+++&&&+++$
**|||****$
=!!!=====$
::::::::#$
........@$
         $$"#,
"",
"",
"",
"",
"",
"",
r#"
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&&+$
||||||||*$
!!!==!!!=$
:::##:::#$
...@@...@$
         $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&++$
||||||||*$
!!!==!!!=$
::::::::#$
.......@@$
         $$"#,
r#"
.eeeee.$
@@@@@@:$
%%%----$
&&&++++$
|||****$
!!!====$
::::::#$
@.....@$
       $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
::::::::#$
.......@@$
         $$"#,
r#"
eeeeee.$
@@@@@@:$
%%%----$
&&&&&++$
|||||**$
!!!====$
::::::#$
......@$
       $$"#,
r#"
eeeeee.$
@@@@@@:$
%%%----$
&&&&&++$
|||||**$
!!!====$
:::####$
...@@@@$
       $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%------$
&&&++++++$
|||*||||*$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
eee..eee.$
@@@::@@@:$
%%%--%%%-$
&&&&&&&&+$
||||||||*$
!!!==!!!=$
:::##:::#$
...@@...@$
         $$"#,
r#"
eee.$
@@@:$
%%%-$
&&&+$
|||*$
!!!=$
:::#$
...@$
    $$"#,
r#"
eeeeeee.$
@@@@@@@:$
----%%%-$
++++&&&+$
****|||*$
!!==!!!=$
:::::::#$
@.....@@$
        $$"#,
r#"
eee..eee.$
@@@::@@@:$
%%%-%%%--$
&&&&&&+++$
||||||***$
!!!=!!!==$
:::##:::#$
...@@...@$
         $$"#,
r#"
eee....$
@@@::::$
%%%----$
&&&++++$
|||****$
!!!====$
::::::#$
......@$
       $$"#,
r#"
eee......eee.$
@@@@::::@@@@:$
%%%%%--%%%%%-$
&&&&&&&&&&&&+$
|||*||||*|||*$
!!!==!!==!!!=$
:::######:::#$
...@@@@@@...@$
             $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
:::##:::#$
...@@...@$
         $$"#,
r#"
.eeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&&+$
|||||||**$
!!!======$
:::######$
...@@@@@@$
         $$"#,
r#"
.eeeeeeee...$
@@@@@@@@@@::$
%%%----%%%--$
&&&++++&&&++$
|||*||*|||**$
!!!==!!!!!==$
::::::::::##$
@......@...@$
            $$"#,
r#"
eeeeeee..$
@@@@@@@@:$
%%%--%%%-$
&&&&&&&++$
||||||***$
!!!=!!!==$
:::##:::#$
...@@...@$
         $$"#,
r#"
.eeeeee.$
@@@@@@@:$
%%%-----$
&&&&&&++$
*||||||*$
====!!!=$
:::::::#$
......@@$
        $$"#,
r#"
eeeeeeeee.$
@@@@@@@@@:$
---%%%----$
+++&&&++++$
***|||****$
===!!!====$
###:::####$
@@@...@@@@$
          $$"#,
r#"
eee..eee.$
@@@::@@@:$
%%%--%%%-$
&&&++&&&+$
|||**|||*$
!!!==!!!=$
::::::::#$
@......@@$
         $$"#,
r#"
eee....eee.$
@@@::::@@@:$
%%%----%%%-$
&&&++++&&&+$
*|||**|||**$
==!!!!!!===$
###::::####$
@@@@..@@@@@$
           $$"#,
r#"
eee..eee..eee.$
@@@::@@@::@@@:$
%%%--%%%--%%%-$
&&&++&&&++&&&+$
|||**|||**|||*$
!!!==!!!==!!!=$
:::::::::::::#$
@...........@@$
              $$"#,
r#"
eee..eee.$
@@@::@@@:$
%%%--%%%-$
+&&&&&&++$
*||||||**$
!!!==!!!=$
:::##:::#$
...@@...@$
         $$"#,
r#"
eee...eee.$
@@@:::@@@.$
%%%---%%%-$
+&&&&&&&++$
**|||||***$
===!!!====$
###:::####$
@@@...@@@@$
          $$"#,
r#"
eeeeeeee.$
@@@@@@@@:$
----%%%--$
+++&&&+++$
**|||****$
=!!!=====$
::::::::#$
........@$
         $$"#,
"",
"",
"",
"",
];    
