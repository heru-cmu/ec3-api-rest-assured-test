import static org.assertj.core.api.Assertions.*
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper

RequestObject request = findTestObject('Petstore/PUT User (Update)')

// Override URL dengan username valid
request.setRestUrl("https://petstore.swagger.io/v2/user/heruQA3")

// Body valid untuk update
request.setBodyContent(new com.kms.katalon.core.testobject.impl.HttpTextBodyContent(
    '''{
        "id": 123,
        "username": "heruQA3",
        "firstName": "Heru",
        "lastName": "Utama",
        "email": "heru.updated@test.com",
        "password": "newPass123",
        "phone": "08123456789",
        "userStatus": 1
    }''', "UTF-8", "application/json"))

ResponseObject response = WS.sendRequest(request)
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

def jsonResponse = new JsonSlurper().parseText(response.getResponseBodyContent())
assert jsonResponse.code instanceof Integer
assert jsonResponse.type instanceof String
assert jsonResponse.message != null

println("✅ Positive PUT User berhasil diverifikasi")
println("Response body: " + jsonResponse)
