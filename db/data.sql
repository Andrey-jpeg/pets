CREATE TABLE Pets (
    PetID int PRIMARY KEY NOT NULL AUTO_INCREMENT,
    PetName varchar(255) NOT NULL,
    Color varchar(255),
    Status varchar(2555)
);
CREATE TABLE Users (
    UserID int PRIMARY KEY NOT NULL AUTO_INCREMENT,
    FirstName varchar(255) NOT NULL,
    LastName varchar(255) NOT NULL,
    Username varchar(255) NOT NULL UNIQUE,
    Password varchar(255) NOT NULL,
    PhoneNumber varchar(20),
    email varchar(255)
);
CREATE TABLE UserPets (
	PetID INT,
    UserID INT,
    FOREIGN KEY (PetID) REFERENCES Pets(PetID),
    FOREIGN KEY (UserID) REFERENCES Users(UserID),
    PRIMARY KEY(PetID, UserID)
);
CREATE TABLE PetLocation (
    PetID INT,
    Latitude float not null,
    Longitude float not null,
    PRIMARY KEY (PetID),
    FOREIGN KEY (PetID) REFERENCES Pets(PetID)
);

INSERT INTO Pets(PetName, Color, Status)
VALUES ("Cookie", "Sort og hvid", "Active");

INSERT INTO Users(UserID, FirstName, LastName, Username, Password)
VALUES ("Hans", "Hansen", "H4N5", "5N4H");

INSERT INTO UserPets(PetID, UserID)
VALUES (1, 1);