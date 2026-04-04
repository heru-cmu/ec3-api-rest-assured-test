<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT User (Update)</name>
   <tag></tag>
   <elementGuidId>adddf25e-5185-45f5-831d-7ce464dc492b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: 9223372036854756858,\n  \&quot;username\&quot;: \&quot;heruQA3\&quot;,\n  \&quot;firstName\&quot;: \&quot;Heru3\&quot;,\n  \&quot;lastName\&quot;: \&quot;Utama3\&quot;,\n  \&quot;email\&quot;: \&quot;heru3@test.com\&quot;,\n  \&quot;password\&quot;: \&quot;12345\&quot;,\n  \&quot;phone\&quot;: \&quot;08123456789\&quot;,\n  \&quot;userStatus\&quot;: 1\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ee1c1c51-932e-473c-9e52-28efc439ced8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.0.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user/heruQA3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
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
def schemaFile = new File(projectDir + &quot;/Include/resources/schema/put-user-schema.json&quot;)
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

println(&quot;✅ PUT User response valid sesuai schema&quot;)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
