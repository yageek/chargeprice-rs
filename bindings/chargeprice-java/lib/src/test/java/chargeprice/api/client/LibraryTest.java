/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package chargeprice.api.client;

import org.junit.Test;
import static org.junit.Assert.*;

public class LibraryTest {
    @Test public void testSomeAPI() {

        Client client;
        try {
            client = new Client("some_key");

            client.close();
        } catch (Throwable throwable) {
            throwable.printStackTrace();
        }


    }
}
