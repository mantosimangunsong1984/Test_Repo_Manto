<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Supporting Document                    _e4ebcb</name>
   <tag></tag>
   <elementGuidId>79a56879-643d-4a7d-929e-ae2835fcb306</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='purchase_request']/div/div/div[2]/div[9]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value></value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;Supporting Document Choose file... Maks.: 10 MB jpg, jpeg, png, tiff, pdf, doc, &quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>baba7ae0-c63e-47b1-92c8-f0b5aeab6542</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>col-md-6 fv-row mb-9</value>
      <webElementGuid>e1f67d84-1135-409d-9846-eb939347133d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                
                    Supporting Document
                
                
                
                                    
                        
                            
                                
                                Choose file...
                                Maks.: 10 MB jpg, jpeg, png, tiff, pdf, doc, docx, xls, xlsx
                            
                        
                    
                    
                        $(document).ready(function() {
                            var element_id = 'supporting_document_1';
                            validation_upload_file_text(element_id);
                            $('#'+element_id).on('change', function() {validation_upload_file(element_id)});
                        });
                    
                                
                    $(document).on('click','.btn-delsp',function(e){
                        e.preventDefault()
                        var id = $(this).parent().parent().last()[0].id
                        var split_last_id = id.split('_').pop()
                        if(split_last_id>1){
                            $(this).parent().parent().remove()
                        }else{
                            $(this).parent().parent().find('.inputan').find('label').text('Choose file...')
                            $(this).parent().parent().find('.inputan').find('input').val('')
                            $(this).parent().parent().parent().find('input').val('')
                            $(this).parent().parent().find('.inputan').find('a').remove()
                        }
                    });
                
                
                    
                    
                    
                    
                        
                            
                            
                        
                    
                    Add Supporting Document
                    
                
            </value>
      <webElementGuid>711113dc-94b4-4cec-abe2-3f7d2dd0c932</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;purchase_request&quot;)/div[@class=&quot;card mb-10&quot;]/div[@class=&quot;card-body&quot;]/div[@class=&quot;row content-tab-home mt-4&quot;]/div[@class=&quot;col-md-6 fv-row mb-9&quot;]</value>
      <webElementGuid>8b405b9a-c836-4e94-b9a8-61d5f0a53581</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='purchase_request']/div/div/div[2]/div[9]</value>
      <webElementGuid>a4285663-ec1c-4959-8da3-0ac6deed96c2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Ari Rahmat'])[3]/following::div[1]</value>
      <webElementGuid>ddb03eba-da30-44bd-969c-3a815d81ed8f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='*'])[7]/following::div[1]</value>
      <webElementGuid>69d593dc-66b8-4fb9-8fe0-8ffd369c2efc</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[9]</value>
      <webElementGuid>0c5213cd-6b20-423b-84a0-1cded99abc45</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
                
                    Supporting Document
                
                
                
                                    
                        
                            
                                
                                Choose file...
                                Maks.: 10 MB jpg, jpeg, png, tiff, pdf, doc, docx, xls, xlsx
                            
                        
                    
                    
                        $(document).ready(function() {
                            var element_id = &quot; , &quot;'&quot; , &quot;supporting_document_1&quot; , &quot;'&quot; , &quot;;
                            validation_upload_file_text(element_id);
                            $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+element_id).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {validation_upload_file(element_id)});
                        });
                    
                                
                    $(document).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;.btn-delsp&quot; , &quot;'&quot; , &quot;,function(e){
                        e.preventDefault()
                        var id = $(this).parent().parent().last()[0].id
                        var split_last_id = id.split(&quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot;).pop()
                        if(split_last_id>1){
                            $(this).parent().parent().remove()
                        }else{
                            $(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;.inputan&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Choose file...&quot; , &quot;'&quot; , &quot;)
                            $(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;.inputan&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                            $(this).parent().parent().parent().find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                            $(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;.inputan&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;).remove()
                        }
                    });
                
                
                    
                    
                    
                    
                        
                            
                            
                        
                    
                    Add Supporting Document
                    
                
            &quot;) or . = concat(&quot;
                
                    Supporting Document
                
                
                
                                    
                        
                            
                                
                                Choose file...
                                Maks.: 10 MB jpg, jpeg, png, tiff, pdf, doc, docx, xls, xlsx
                            
                        
                    
                    
                        $(document).ready(function() {
                            var element_id = &quot; , &quot;'&quot; , &quot;supporting_document_1&quot; , &quot;'&quot; , &quot;;
                            validation_upload_file_text(element_id);
                            $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+element_id).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {validation_upload_file(element_id)});
                        });
                    
                                
                    $(document).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;.btn-delsp&quot; , &quot;'&quot; , &quot;,function(e){
                        e.preventDefault()
                        var id = $(this).parent().parent().last()[0].id
                        var split_last_id = id.split(&quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot;).pop()
                        if(split_last_id>1){
                            $(this).parent().parent().remove()
                        }else{
                            $(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;.inputan&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Choose file...&quot; , &quot;'&quot; , &quot;)
                            $(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;.inputan&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                            $(this).parent().parent().parent().find(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
                            $(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;.inputan&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;).remove()
                        }
                    });
                
                
                    
                    
                    
                    
                        
                            
                            
                        
                    
                    Add Supporting Document
                    
                
            &quot;))]</value>
      <webElementGuid>b568bfeb-c320-42bf-b7fc-e95e25a15c19</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
