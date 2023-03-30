CREATE TABLE Pets (
    PetID int PRIMARY KEY NOT NULL,
    PetName varchar(255),
    Color varchar(255),
    Status varchar(2555)
);

CREATE TABLE PetsByOwners (
    FOREIGN KEY (PetID) REFERENCES Pets(PetID),
    FOREIGN KEY (PersonID) REFERENCES Owners(PersonID)
);

CREATE TABLE Owners (
    PersonID int PRIMARY KEY NOT NULL,
    FirstName varchar(255),
    LastName varchar(255)
);


