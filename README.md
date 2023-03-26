# uprn_match

This respository is a quick test of the viability of matching UPRNs to the freetext addresses of Care Homes in Scotland recorded in the Care Inspectorate DataStore.

The code in this repository depends on the existence of two files in the `data/` directory that are not included in this GitHub repo because this repo is a not a controller of the data.

The file `data/ci_datastore.csv` can be downloaded from: [Datastore](https://www.careinspectorate.com/index.php/statistics-and-analysis/data-and-analysis) and renamed to `ci_datastore.csv`.

The file `data/addressbase_premium_scotland.gpkg` can be downloaded from [OS Data downloads | OS Data Hub](https://osdatahub.os.uk/downloads/packages) if you have a Premium Plan with the Ordnance Survey or are recognised on the Public Sector Plan which gives free access to this data.  The file is listed on the site as "AddressBase Premium - SCOTLAND" and is a zipped SQLite file conforming to the OGC GeoPackage standard.

The code in this repository is very simple.  It contains code that can read all the addresses from the Care Inspectorate DataStore CSV file and code that can read all of the `delivery_point_address` entries from AddressBase Premium for Scotland.  It uses the postcode entry to sub-select addresses that match the correct postcode and then it then uses one of a number of string similarity measures to match registered addresses with those in the Inspectorate Database and displays the matches.

Currently the code does nothing more than testing the first forty addresses against manually assessed correct matches to try to establish the accuracy of this approach and provide a playground for testing alternative techniques.  It currently correctly matches 26 of the 40 addresses by simply choosing the most likely match of those it can find.

At this point it has been establishes that the string similarity measure is giving insufficient priority to the building number.  Fixing this is likely to significantly increase the number of matches in the first 40 addresses.

It is also notable that one of the addresses in the forty does not appear - on manual testing to have a valid UPRN and others are so poorly described that it seems unrealistic to expect them to be matched automatically.
