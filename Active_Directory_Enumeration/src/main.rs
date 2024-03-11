// Import necessary modules from the ldap3 crate
use ldap3::*;

// Entry point of the program
fn main() {
    // Define the LDAP server URL
    let ldap_url = "ldap://192.168.0.110:3268";

    // Attempt to establish a connection to the LDAP server
    let ldap = LdapConn::new(ldap_url);

    // Match the result of the LDAP connection attempt
    let mut ldap_conn: LdapConn = match ldap {
        // If successful, assign the connection to ldap_conn
        Ok(l) => l,
        // If an error occurs, panic with an error message
        Err(e) => panic!("Failed to connect to LDAP server: {e}"),
    };

    // Attempt to bind to the LDAP server using a username and password
    if let Err(e) = ldap_conn.simple_bind("CN=Administrator,CN=Users,DC=tech69,DC=local", "Passw0rd") {
        // If binding fails, panic with an error message
        panic!("Failed to bind to LDAP server: {e}");
    }

    // Define the username filter for LDAP search
    let username = "*)(serviceprincipalname=*";

    // Construct the LDAP filter string based on the username
    let filter = format!("(&(objectclass=user)(samaccountname={username})))");

    // Print the LDAP filter for debugging purposes
    println!("Filter: {}", filter);

    // Perform an LDAP search operation with the constructed filter
    let search_result = ldap_conn.search("DC=tech69,DC=local", Scope::Subtree, &filter, vec!["dn"]);

    // Match the result of the LDAP search operation
    match search_result {
        // If the search is successful, process the search results
        Ok(result) => {
            // Extract the search entries and LDAP result from the search result
            let (entries, ldap_result) = result.success().unwrap();

            // Print the LDAP result for debugging purposes
            println!("LDAP Result: {:?}", ldap_result);
            
            // Iterate over each search entry and print its distinguished name (DN)
            for entry in entries {
                println!("DN: {:?}", SearchEntry::construct(entry).dn);
            }
        }
        // If an error occurs during the LDAP search, print the error message
        Err(e) => {
            println!("Error occurred during LDAP search: {e}");
        }
    }
}
