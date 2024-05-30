-- Respect minimum salary.
DROP PROCEDURE assume_above_min_sal;
DROP TRIGGER respectMinSalWhenUpdating;
DROP TRIGGER respectMinSalWhenInserting;
-- Enforce relative validity of dates.
DROP PROCEDURE assume_rel_validity_of_dates;
DROP TRIGGER respectRelOrdOfDatesWhenUpdating;
DROP TRIGGER respectRelOrdOfDatesWhenInserting;