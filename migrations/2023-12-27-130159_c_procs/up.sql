CREATE PROCEDURE make_order(IN project_id int, IN client_id int, IN pay decimal(12, 2),
                            IN spectrum_of_interest varchar(800))
INSERT INTO Client_Project (project_id, client_id, pays, spectrum_of_interest)
VALUES (project_id, client_id, pays, spectrum_of_interest);

CREATE FUNCTION get_min_sal() RETURNS decimal(12, 2)
    RETURN (SELECT val FROM MIN_SALARY);

CREATE PROCEDURE set_min_sal(IN new_min_sal decimal(12,2))
BEGIN
    -- Update minimum salary constant.
    UPDATE MIN_SALARY SET val = new_min_sal;

    -- Raise salaries if they are below the new minimum
    UPDATE Employee SET salary = new_min_sal WHERE salary < new_min_sal;
END;

CREATE FUNCTION avg_salary()
RETURNS decimal(12,2)
RETURN (SELECT AVG(salary) FROM Employee);