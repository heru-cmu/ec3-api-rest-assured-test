<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET User</name>
   <tag></tag>
   <elementGuidId>fcd80cfa-8108-468a-97c2-f5316171ebf9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
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
      <webElementGuid>1a8652dd-45ab-48d1-9785-bb7b6de156bd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.0.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user/heruQA3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*&#xd;
import com.kms.katalon.core.testobject.ResponseObject&#xd;
import com.kms.katalon.core.webservice.verification.WSResponseManager&#xd;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS&#xd;
import com.kms.katalon.core.configuration.RunConfiguration&#xd;
import groovy.json.JsonSlurper&#xd;
&#xd;
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()&#xd;
&#xd;
// 1. Status code check&#xd;
WS.verifyResponseStatusCode(response, 200)&#xd;
assertThat(response.getStatusCode()).isEqualTo(200)&#xd;
&#xd;
// 2. Parse response&#xd;
def slurper = new JsonSlurper()&#xd;
def jsonResponse = slurper.parseText(response.getResponseBodyContent())&#xd;
&#xd;
// 3. Load schema file&#xd;
def projectDir = RunConfiguration.getProjectDir()&#xd;
def schemaFile = new File(projectDir + &quot;/Include/resources/schema/get-user-schema.json&quot;)&#xd;
def schema = slurper.parseText(schemaFile.text)&#xd;
&#xd;
// 4. Validasi field sesuai schema (strict)&#xd;
schema.properties.each { key, value ->&#xd;
    assert jsonResponse.containsKey(key) : &quot;Field '${key}' tidak ada di response&quot;&#xd;
    def actualType = jsonResponse[key]?.getClass()?.getSimpleName()?.toLowerCase()&#xd;
    assert actualType.contains(value.type) : &quot;Field '${key}' tipe salah. Dapat: ${actualType}, Harus: ${value.type}&quot;&#xd;
    println(&quot;Field '${key}' OK sebagai ${actualType}&quot;)&#xd;
}&#xd;
&#xd;
// 5. Required field check&#xd;
schema.required.each { req ->&#xd;
    assert jsonResponse[req] != null : &quot;Field '${req}' wajib ada tapi null&quot;&#xd;
}&#xd;
&#xd;
println(&quot;✅ GET User response valid sesuai schema (strict)&quot;)&#xd;
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
