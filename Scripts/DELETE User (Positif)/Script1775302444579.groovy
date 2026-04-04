import static org.assertj.core.api.Assertions.*
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper

RequestObject request = findTestObject('Petstore/DELETE User')
request.setRestUrl("https://petstore.swagger.io/v2/user/heruQA3")

ResponseObject response = WS.sendRequest(request)
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

def jsonResponse = new JsonSlurper().parseText(response.getResponseBodyContent())
assert jsonResponse.code instanceof Integer
assert jsonResponse.type instanceof String
assert jsonResponse.message == "heruQA3"

println("✅ Positive DELETE User berhasil diverifikasi")
println("Response body: " + jsonResponse)
