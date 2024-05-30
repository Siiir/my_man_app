-- Respect minimum salary.

CREATE PROCEDURE assume_above_min_sal(salary DECIMAL(12, 2))
BEGIN
    DECLARE min_salary DECIMAL(12, 2);
    DECLARE err_msg VARCHAR(200);

    SET min_salary = get_min_sal();
    SET err_msg = CONCAT(
            'Salary cannot be below the minimum salary, which equals ',
            min_salary,
            '.');

    IF salary < min_salary THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = err_msg;
    END IF;
END;

-- Create a trigger for BEFORE INSERT
CREATE TRIGGER respectMinSalWhenInserting
    BEFORE INSERT
    ON Employee
    FOR EACH ROW
    CALL assume_above_min_sal(NEW.salary);

-- Create a trigger for BEFORE UPDATE
CREATE TRIGGER respectMinSalWhenUpdating
    BEFORE UPDATE
    ON Employee
    FOR EACH ROW
    CALL assume_above_min_sal(NEW.salary);

-- Enforce relative validity of dates.

CREATE PROCEDURE assume_rel_validity_of_dates(IN start_date DATE, IN end_date DATE, IN deadline TIMESTAMP)
BEGIN
    -- Check if end_date is not null and start_date is greater than end_date
    IF end_date IS NOT NULL AND start_date > end_date THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Start date must not be after end date.';
    END IF;
    -- Check if start_date is greater than deadline
    IF start_date > DATE(deadline) THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Start date must not be after deadline.';
    END IF;
END;

CREATE TRIGGER respectRelOrdOfDatesWhenUpdating BEFORE UPDATE ON Project
    FOR EACH ROW CALL assume_rel_validity_of_dates(NEW.start_date, NEW.end_date, NEW.deadline);

CREATE TRIGGER respectRelOrdOfDatesWhenInserting BEFORE INSERT ON Project
    FOR EACH ROW CALL assume_rel_validity_of_dates(NEW.start_date, NEW.end_date, NEW.deadline);