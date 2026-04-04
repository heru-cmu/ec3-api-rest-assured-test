<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DELETE User</name>
   <tag></tag>
   <elementGuidId>66bd3a62-d594-44e7-8e37-d26b5610b735</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>27c8da41-fec9-4590-ba11-af8529984784</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user/heruQA3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.configuration.RunConfiguration
import groovy.json.JsonSlurper

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// 1. Status code check
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// 2. Parse response
def slurper = new JsonSlurper()
def jsonResponse = slurper.parseText(response.getResponseBodyContent())

// 3. Load schema file
def projectDir = RunConfiguration.getProjectDir()
def schemaFile = new File(projectDir + &quot;/Include/resources/schema/delete-user-schema.json&quot;)
def schema = slurper.parseText(schemaFile.text)

// 4. Validasi field sesuai schema (strict)
schema.properties.each { key, value ->
    assert jsonResponse.containsKey(key) : &quot;Field '${key}' tidak ada di response&quot;
    def actualType = jsonResponse[key]?.getClass()?.getSimpleName()?.toLowerCase()
    assert actualType.contains(value.type) : &quot;Field '${key}' tipe salah. Dapat: ${actualType}, Harus: ${value.type}&quot;
    println(&quot;Field '${key}' OK sebagai ${actualType}&quot;)
}

// 5. Required field check
schema.required.each { req ->
    assert jsonResponse[req] != null : &quot;Field '${req}' wajib ada tapi null&quot;
}

println(&quot;✅ DELETE User response valid sesuai schema&quot;)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
