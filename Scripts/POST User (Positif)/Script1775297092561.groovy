import static org.assertj.core.api.Assertions.*
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper

// Panggil request object dari Object Repository
TestObject request = findTestObject('Petstore/POST User (Register)')

// Kirim request
ResponseObject response = WS.sendRequest(request)

// Assertion status code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// Parse body JSON
def jsonResponse = new JsonSlurper().parseText(response.getResponseBodyContent())

// Assertion isi body
assert jsonResponse.code instanceof Integer : "code harus integer"
assert jsonResponse.type instanceof String : "type harus string"
assert jsonResponse.message != null : "message tidak boleh null"

// Branching hasil
if (jsonResponse.code == 200 && jsonResponse.message != null) {
    println("✅ POST User berhasil diverifikasi")
    println("Response body: " + jsonResponse)
} else {
    println("❌ POST User gagal diverifikasi")
    println("Response body: " + jsonResponse)
}
