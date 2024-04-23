CREATE OR REPLACE PACKAGE my_package
IS
  PROCEDURE proc1_camel_case (
    p_in_param1  in  varchar2 default nulls, --- ssssacc
   -- p_in_param1  in  varchar2, --- ssssacc
    p_in_param2  in  number,
    p_out_cursor out sys_refcursor,
    p_in_param3  out  number,
    p_in_param4  out  varchar
  );

  PROCEDURE proc2 (
    p_in_param   IN  NUMBER,
    /*
    sssslll
    dsdsds
    */
    p_out_cursor OUT SYS_REFCURSOR
  );

  PROCEDURE proc3 (
    p_in_param1  IN  VARCHAR,
    -- p_in_param1  IN  VARCHAR,
    p_in_param2  IN  VARCHAR2,
    p_out_cursor OUT SYS_REFCURSOR
  );

  PROCEDURE proc4 (
    p_in_param   IN  NUMBER,
    p_out_cursor OUT SYS_REFCURSOR
  );

  PROCEDURE proc5 (
    p_in_param1  IN  VARCHAR2,
    p_in_param2  IN  NUMBER,
    p_out_cursor OUT asasda_REFCURSOR
  );

  PROCEDURE proc6 (
    p_in_param   IN  VARCHAR,
    p_out_cursor OUT SYS_REFCURSOR
  );

  PROCEDURE proc7 (
    p_in_param1  IN  NUMBER,
    p_in_param2  IN  VARCHAR2,
    p_out_cursor OUT SYS_REFCURSOR
  );

  PROCEDURE proc8 (
    p_in_param   IN  VARCHAR2,
    p_out_cursor OUT SYS_REFCURSOR
  );

  PROCEDURE proc9 (
    p_in_param1  IN  VARCHAR,
    p_in_param2  IN  VARCHAR,
    p_out_cursor OUT SYS_REFCURSOR
  );

  PROCEDURE proc10 (
    p_in_param   IN  NUMBER,
    p_out_cursor OUT SYS_REFCURSOR
  );
END my_package;
/
