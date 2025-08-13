<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Sign Out_kt_content</name>
   <tag></tag>
   <elementGuidId>9fbb73c7-7361-4587-81fc-39b90454b1c1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#kt_content</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='kt_content']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#kt_content</value>
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
      <webElementGuid>b145c83b-3517-47eb-b8e8-b48b0d71f0a7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>content d-flex flex-column flex-column-fluid</value>
      <webElementGuid>c2150565-8f25-48a3-87ea-05abac700b96</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>kt_content</value>
      <webElementGuid>11c9b8c9-b16e-4911-a294-4460fed4ea58</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
						
						
							
														


	
	

		
		
			
			
				
				
					
						
							
							
							
						
					
				
				
				
								
				
				
				
					
						
							
							
							
						
					
				
				Advance Search
				
							
			
		
		
		
		
			
			
								
								
				
				
				
				
				
				
																																
			
			
			
				
				Selected
				Delete Selected
			
			
			
			
				
				
					
					
						
						
							
							Advance Search
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
												
						
							
							
								
                                    
                                        
                                            
																																					
													
														
														
														
														
																													
																												
					
			
				
				
					UR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Title
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Remarks
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Status
					
				
				
				
				-Select Status-DraftOpenRejectedRevisedFully ApprovedCancelledClosedGood IssuedSelect Status...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Total PR
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Purchasing Group
					
				
				
				
				-Select Purchasing Group-200 - Non-Project201 - ProjectSelect Purchasing Group...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Creator
					
				
				
				
				-Select Creator-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Creator...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Requestor
					
				
				
				
				-Select Requestor-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Requestor...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Department
					
				
				
				
				-Select Department-SUSTAINABILITY &amp; PROJECTSUSTAINABILITYSUSTAINABILITY &amp; PROJECT - PROJECTCORPORATE RELATIONTREASURY &amp; BANKING RELATIONTREASURY &amp; BANKING RELATION - DEPTFINANCE, TAX &amp; ACCOUNTINGFINANCE OPERATION ACCOUNTING &amp; TAXFINANCE STRATEGIC REPORTING &amp; BUDGET CONTROLINTERNAL AUDIT &amp; RISK MANAGEMENTINTERNAL AUDITRISK MANAGEMENTSTRATEGIC &amp; BUSINESS PROCESS MANAGEMENTSTRATEGIC MANAGEMENT OFFICEBUSINESS PROCESS, IMPROVEMENT &amp; AUTOMATIONLAND &amp; ASSETSCOMPLIANCELEGAL BUSINESS &amp; FINANCINGINFORMATION TECHNOLOGYIT INFRASTRUCTURESERVICE MANAGEMENT &amp; SUPPORTIT BUSINESS PARTNER &amp; BUSINESS ANALYSTIT DEVELOPMENTGENERAL AFFAIRHUMAN RESOURCES &amp; GENERAL AFFAIRHR CENTRE OF EXCELLENCEHR COE - OD &amp; PMHR COE - DATA ANALYTICSHR SHARED SERVICEHR SS - PAYROLL &amp; BENEFITHR SS - HRISEXECUTIVE ASSISTANTHR SS - L&amp;DPERSONAL ASSISTANTBUSINESS DEVELOPMENT HOTREASURY &amp; BANKING PERSONAL ASSISTANTCORPORATE COMMUNICATION &amp; EVENTCORPORATE COMMUNICATIONMEDIA RELATIONASSET MANAGEMENT &amp; PROCUREMENTASSIGNMENTFounder OfficeHR - Organization DevelopmentHR - HRISHR - Reward &amp; Employee ServiceHR - Talent ManagementIT SECURITY AND COMPLIANCESelect Department...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Cost Center
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					On Bidding
					
				
				
				
				-Select On Bidding-YesNoSelect On Bidding...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Bidding Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					PO Fully Created
					
				
				
				
				-Select PO Fully Created-YesNoSelect PO Fully Created...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PO Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Created At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#created_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#created_at&quot;).on('apply.daterangepicker', function(ev, picker) {
					$(this).val(picker.startDate.format('YYYY-MM-DD') + ' - ' + picker.endDate.format('YYYY-MM-DD'));
				});

				$(&quot;#created_at&quot;).on('cancel.daterangepicker', function(ev, picker) {
					$(this).val('');
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Updated At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#updated_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#updated_at&quot;).on('apply.daterangepicker', function(ev, picker) {
					$(this).val(picker.startDate.format('YYYY-MM-DD') + ' - ' + picker.endDate.format('YYYY-MM-DD'));
				});

				$(&quot;#updated_at&quot;).on('cancel.daterangepicker', function(ev, picker) {
					$(this).val('');
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																				
                                        
                                    
                                
								
								
									Close
									Clear
									
										Search
										Please wait...
										
									
								
								
							
							
						
						
											
					
				
				
			
			
			
			
				
				
					
					
						
						
							
							Export Users
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
						
						
							
							
								
								
									
									Select Roles:
									
									
									
										
										Administrator
										Analyst
										Developer
										Support
										Trial
									Select a role
									
								
								
								
								
									
									Select Export Format:
									
									
									
										
										Excel
										PDF
										CVS
										ZIP
									Select a format
									
								
								
								
								
									Discard
									
										Submit
										Please wait...
										
									
								
								
							
							
						
						
					
					
				
				
			
			
			
			
				
				
					
					
						
						
							
							Create New User
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
						
						
							
							
								
								
									
									
										
										Avatar
										
										
										
											
											
											
											
											
												
												
												
												
												
											
											
											
											
												
											
											
											
											
												
											
											
										
										
										
										Allowed file types: png, jpg, jpeg.
										
									
									
									
									
										
										Full Name
										
										
										
										
									
									
									
									
										
										Email
										
										
										
										
									
									
									
									
										
										Role
										
										
										
										
											
											
												
												
												
												
												
													Administrator
													Best for business owners and company administrators
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Developer
													Best for developers or people primarily using the API
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Analyst
													Best for people who need full access to analytics data, but don't need to update business settings
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Support
													Best for employees who regularly refund payments and respond to disputes
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Trial
													Best for people who need to preview content data, but don't need to make any updates
												
												
											
											
										
										
										
									
									
								
								
								
								
									Discard
									
										Submit
										Please wait...
										
									
								
								
							
							
						
						
					
					
				
				
			
			
		
		
	
	
	
	
				

			
			
				Actions
						
						#
					UR NumberPR NumberTitleRemarksStatusTotal PRPurchasing GroupCreatorRequestorDepartmentCost CenterOn BiddingBidding NumberPO Fully CreatedPO NumberCreated AtUpdated At
				
				Actions
						
						#
					UR NumberPR NumberTitleRemarksStatusTotal PRPurchasing GroupCreatorRequestorDepartmentCost CenterOn BiddingBidding NumberPO Fully CreatedPO NumberCreated AtUpdated At
				
				
				
				No data available in table
			Processing...102550100Showing no records
			
		
	
	


							
						
						
					</value>
      <webElementGuid>3c184b03-e75f-4f33-bcf8-5002e546fc24</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;kt_content&quot;)</value>
      <webElementGuid>a7f27501-9de7-4c74-921e-4218a030453b</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='kt_content']</value>
      <webElementGuid>04197fbe-15d1-4639-90f9-bc82cc949c46</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='kt_wrapper']/div[2]</value>
      <webElementGuid>0a8eeeb7-2798-4a40-baac-9cc99a00ca03</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div[2]</value>
      <webElementGuid>a40c09e0-6b28-460d-86d7-264cb0a68a68</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'kt_content' and (text() = concat(&quot;
						
						
							
														


	
	

		
		
			
			
				
				
					
						
							
							
							
						
					
				
				
				
								
				
				
				
					
						
							
							
							
						
					
				
				Advance Search
				
							
			
		
		
		
		
			
			
								
								
				
				
				
				
				
				
																																
			
			
			
				
				Selected
				Delete Selected
			
			
			
			
				
				
					
					
						
						
							
							Advance Search
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
												
						
							
							
								
                                    
                                        
                                            
																																					
													
														
														
														
														
																													
																												
					
			
				
				
					UR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Title
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Remarks
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Status
					
				
				
				
				-Select Status-DraftOpenRejectedRevisedFully ApprovedCancelledClosedGood IssuedSelect Status...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Total PR
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Purchasing Group
					
				
				
				
				-Select Purchasing Group-200 - Non-Project201 - ProjectSelect Purchasing Group...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Creator
					
				
				
				
				-Select Creator-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Creator...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Requestor
					
				
				
				
				-Select Requestor-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Requestor...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Department
					
				
				
				
				-Select Department-SUSTAINABILITY &amp; PROJECTSUSTAINABILITYSUSTAINABILITY &amp; PROJECT - PROJECTCORPORATE RELATIONTREASURY &amp; BANKING RELATIONTREASURY &amp; BANKING RELATION - DEPTFINANCE, TAX &amp; ACCOUNTINGFINANCE OPERATION ACCOUNTING &amp; TAXFINANCE STRATEGIC REPORTING &amp; BUDGET CONTROLINTERNAL AUDIT &amp; RISK MANAGEMENTINTERNAL AUDITRISK MANAGEMENTSTRATEGIC &amp; BUSINESS PROCESS MANAGEMENTSTRATEGIC MANAGEMENT OFFICEBUSINESS PROCESS, IMPROVEMENT &amp; AUTOMATIONLAND &amp; ASSETSCOMPLIANCELEGAL BUSINESS &amp; FINANCINGINFORMATION TECHNOLOGYIT INFRASTRUCTURESERVICE MANAGEMENT &amp; SUPPORTIT BUSINESS PARTNER &amp; BUSINESS ANALYSTIT DEVELOPMENTGENERAL AFFAIRHUMAN RESOURCES &amp; GENERAL AFFAIRHR CENTRE OF EXCELLENCEHR COE - OD &amp; PMHR COE - DATA ANALYTICSHR SHARED SERVICEHR SS - PAYROLL &amp; BENEFITHR SS - HRISEXECUTIVE ASSISTANTHR SS - L&amp;DPERSONAL ASSISTANTBUSINESS DEVELOPMENT HOTREASURY &amp; BANKING PERSONAL ASSISTANTCORPORATE COMMUNICATION &amp; EVENTCORPORATE COMMUNICATIONMEDIA RELATIONASSET MANAGEMENT &amp; PROCUREMENTASSIGNMENTFounder OfficeHR - Organization DevelopmentHR - HRISHR - Reward &amp; Employee ServiceHR - Talent ManagementIT SECURITY AND COMPLIANCESelect Department...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Cost Center
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					On Bidding
					
				
				
				
				-Select On Bidding-YesNoSelect On Bidding...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Bidding Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					PO Fully Created
					
				
				
				
				-Select PO Fully Created-YesNoSelect PO Fully Created...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PO Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Created At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#created_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#created_at&quot;).on(&quot; , &quot;'&quot; , &quot;apply.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(picker.startDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + picker.endDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;));
				});

				$(&quot;#created_at&quot;).on(&quot; , &quot;'&quot; , &quot;cancel.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Updated At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#updated_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#updated_at&quot;).on(&quot; , &quot;'&quot; , &quot;apply.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(picker.startDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + picker.endDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;));
				});

				$(&quot;#updated_at&quot;).on(&quot; , &quot;'&quot; , &quot;cancel.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																				
                                        
                                    
                                
								
								
									Close
									Clear
									
										Search
										Please wait...
										
									
								
								
							
							
						
						
											
					
				
				
			
			
			
			
				
				
					
					
						
						
							
							Export Users
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
						
						
							
							
								
								
									
									Select Roles:
									
									
									
										
										Administrator
										Analyst
										Developer
										Support
										Trial
									Select a role
									
								
								
								
								
									
									Select Export Format:
									
									
									
										
										Excel
										PDF
										CVS
										ZIP
									Select a format
									
								
								
								
								
									Discard
									
										Submit
										Please wait...
										
									
								
								
							
							
						
						
					
					
				
				
			
			
			
			
				
				
					
					
						
						
							
							Create New User
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
						
						
							
							
								
								
									
									
										
										Avatar
										
										
										
											
											
											
											
											
												
												
												
												
												
											
											
											
											
												
											
											
											
											
												
											
											
										
										
										
										Allowed file types: png, jpg, jpeg.
										
									
									
									
									
										
										Full Name
										
										
										
										
									
									
									
									
										
										Email
										
										
										
										
									
									
									
									
										
										Role
										
										
										
										
											
											
												
												
												
												
												
													Administrator
													Best for business owners and company administrators
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Developer
													Best for developers or people primarily using the API
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Analyst
													Best for people who need full access to analytics data, but don&quot; , &quot;'&quot; , &quot;t need to update business settings
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Support
													Best for employees who regularly refund payments and respond to disputes
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Trial
													Best for people who need to preview content data, but don&quot; , &quot;'&quot; , &quot;t need to make any updates
												
												
											
											
										
										
										
									
									
								
								
								
								
									Discard
									
										Submit
										Please wait...
										
									
								
								
							
							
						
						
					
					
				
				
			
			
		
		
	
	
	
	
				

			
			
				Actions
						
						#
					UR NumberPR NumberTitleRemarksStatusTotal PRPurchasing GroupCreatorRequestorDepartmentCost CenterOn BiddingBidding NumberPO Fully CreatedPO NumberCreated AtUpdated At
				
				Actions
						
						#
					UR NumberPR NumberTitleRemarksStatusTotal PRPurchasing GroupCreatorRequestorDepartmentCost CenterOn BiddingBidding NumberPO Fully CreatedPO NumberCreated AtUpdated At
				
				
				
				No data available in table
			Processing...102550100Showing no records
			
		
	
	


							
						
						
					&quot;) or . = concat(&quot;
						
						
							
														


	
	

		
		
			
			
				
				
					
						
							
							
							
						
					
				
				
				
								
				
				
				
					
						
							
							
							
						
					
				
				Advance Search
				
							
			
		
		
		
		
			
			
								
								
				
				
				
				
				
				
																																
			
			
			
				
				Selected
				Delete Selected
			
			
			
			
				
				
					
					
						
						
							
							Advance Search
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
												
						
							
							
								
                                    
                                        
                                            
																																					
													
														
														
														
														
																													
																												
					
			
				
				
					UR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Title
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Remarks
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Status
					
				
				
				
				-Select Status-DraftOpenRejectedRevisedFully ApprovedCancelledClosedGood IssuedSelect Status...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Total PR
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Purchasing Group
					
				
				
				
				-Select Purchasing Group-200 - Non-Project201 - ProjectSelect Purchasing Group...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Creator
					
				
				
				
				-Select Creator-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Creator...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Requestor
					
				
				
				
				-Select Requestor-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Requestor...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Department
					
				
				
				
				-Select Department-SUSTAINABILITY &amp; PROJECTSUSTAINABILITYSUSTAINABILITY &amp; PROJECT - PROJECTCORPORATE RELATIONTREASURY &amp; BANKING RELATIONTREASURY &amp; BANKING RELATION - DEPTFINANCE, TAX &amp; ACCOUNTINGFINANCE OPERATION ACCOUNTING &amp; TAXFINANCE STRATEGIC REPORTING &amp; BUDGET CONTROLINTERNAL AUDIT &amp; RISK MANAGEMENTINTERNAL AUDITRISK MANAGEMENTSTRATEGIC &amp; BUSINESS PROCESS MANAGEMENTSTRATEGIC MANAGEMENT OFFICEBUSINESS PROCESS, IMPROVEMENT &amp; AUTOMATIONLAND &amp; ASSETSCOMPLIANCELEGAL BUSINESS &amp; FINANCINGINFORMATION TECHNOLOGYIT INFRASTRUCTURESERVICE MANAGEMENT &amp; SUPPORTIT BUSINESS PARTNER &amp; BUSINESS ANALYSTIT DEVELOPMENTGENERAL AFFAIRHUMAN RESOURCES &amp; GENERAL AFFAIRHR CENTRE OF EXCELLENCEHR COE - OD &amp; PMHR COE - DATA ANALYTICSHR SHARED SERVICEHR SS - PAYROLL &amp; BENEFITHR SS - HRISEXECUTIVE ASSISTANTHR SS - L&amp;DPERSONAL ASSISTANTBUSINESS DEVELOPMENT HOTREASURY &amp; BANKING PERSONAL ASSISTANTCORPORATE COMMUNICATION &amp; EVENTCORPORATE COMMUNICATIONMEDIA RELATIONASSET MANAGEMENT &amp; PROCUREMENTASSIGNMENTFounder OfficeHR - Organization DevelopmentHR - HRISHR - Reward &amp; Employee ServiceHR - Talent ManagementIT SECURITY AND COMPLIANCESelect Department...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Cost Center
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					On Bidding
					
				
				
				
				-Select On Bidding-YesNoSelect On Bidding...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Bidding Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					PO Fully Created
					
				
				
				
				-Select PO Fully Created-YesNoSelect PO Fully Created...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PO Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Created At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#created_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#created_at&quot;).on(&quot; , &quot;'&quot; , &quot;apply.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(picker.startDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + picker.endDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;));
				});

				$(&quot;#created_at&quot;).on(&quot; , &quot;'&quot; , &quot;cancel.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Updated At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#updated_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#updated_at&quot;).on(&quot; , &quot;'&quot; , &quot;apply.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(picker.startDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + picker.endDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;));
				});

				$(&quot;#updated_at&quot;).on(&quot; , &quot;'&quot; , &quot;cancel.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																				
                                        
                                    
                                
								
								
									Close
									Clear
									
										Search
										Please wait...
										
									
								
								
							
							
						
						
											
					
				
				
			
			
			
			
				
				
					
					
						
						
							
							Export Users
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
						
						
							
							
								
								
									
									Select Roles:
									
									
									
										
										Administrator
										Analyst
										Developer
										Support
										Trial
									Select a role
									
								
								
								
								
									
									Select Export Format:
									
									
									
										
										Excel
										PDF
										CVS
										ZIP
									Select a format
									
								
								
								
								
									Discard
									
										Submit
										Please wait...
										
									
								
								
							
							
						
						
					
					
				
				
			
			
			
			
				
				
					
					
						
						
							
							Create New User
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
						
						
							
							
								
								
									
									
										
										Avatar
										
										
										
											
											
											
											
											
												
												
												
												
												
											
											
											
											
												
											
											
											
											
												
											
											
										
										
										
										Allowed file types: png, jpg, jpeg.
										
									
									
									
									
										
										Full Name
										
										
										
										
									
									
									
									
										
										Email
										
										
										
										
									
									
									
									
										
										Role
										
										
										
										
											
											
												
												
												
												
												
													Administrator
													Best for business owners and company administrators
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Developer
													Best for developers or people primarily using the API
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Analyst
													Best for people who need full access to analytics data, but don&quot; , &quot;'&quot; , &quot;t need to update business settings
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Support
													Best for employees who regularly refund payments and respond to disputes
												
												
											
											
										
										
										
										
										
											
											
												
												
												
												
												
													Trial
													Best for people who need to preview content data, but don&quot; , &quot;'&quot; , &quot;t need to make any updates
												
												
											
											
										
										
										
									
									
								
								
								
								
									Discard
									
										Submit
										Please wait...
										
									
								
								
							
							
						
						
					
					
				
				
			
			
		
		
	
	
	
	
				

			
			
				Actions
						
						#
					UR NumberPR NumberTitleRemarksStatusTotal PRPurchasing GroupCreatorRequestorDepartmentCost CenterOn BiddingBidding NumberPO Fully CreatedPO NumberCreated AtUpdated At
				
				Actions
						
						#
					UR NumberPR NumberTitleRemarksStatusTotal PRPurchasing GroupCreatorRequestorDepartmentCost CenterOn BiddingBidding NumberPO Fully CreatedPO NumberCreated AtUpdated At
				
				
				
				No data available in table
			Processing...102550100Showing no records
			
		
	
	


							
						
						
					&quot;))]</value>
      <webElementGuid>84a104f8-8acd-4761-a0ec-31f3cf677bca</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
