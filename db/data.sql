CREATE TABLE Pets (
    PetID int PRIMARY KEY NOT NULL,
    PetName varchar(255) NOT NULL,
    Color varchar(255),
    Status varchar(2555)
);
CREATE TABLE Owners (
    PersonID int PRIMARY KEY NOT NULL,
    FirstName varchar(255) NOT NULL,
    LastName varchar(255) NOT NULL,
    PhoneNumber varchar(20),
    email varchar(255)
);
CREATE TABLE PetsByOwners (
	PetID INT,
    PersonID INT,
    FOREIGN KEY (PetID) REFERENCES Pets(PetID),
    FOREIGN KEY (PersonID) REFERENCES Owners(PersonID)
);
CREATE TABLE PetLocation (
    PetID INT,
    Latitude float not null,
    Longitude float not null,
    FOREIGN KEY (PetID) REFERENCES Pets(PetID)
);
INSERT INTO Pets(PetID, PetName, Color, Status)
VALUES (1, "Cookie", "Sort og hvid", "Active");

INSERT INTO Owners(PersonID, FirstName, LastName)
VALUES (1, "Hans", "Hansen");

INSERT INTO PetsByOwners(PetID, PersonID)
VALUES (1, 1);