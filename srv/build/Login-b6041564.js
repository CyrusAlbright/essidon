import{S as s,i as e,v as t,s as a,e as c,a as n,b as l,c as o,C as v,B as r,h as i,D as h,A as d}from"./vendor-4f00c5b8.js";import{n as p}from"./main-1841df6d.js";function u(s){let e,t,h,d,p,u,w,m,q,f,x,g,b,k,C,L,y,M,S,H,T,j,B,D,A,F,P,U,z;return{c(){e=a(),t=c("main"),h=c("a"),h.innerHTML='<svg xmlns="http://www.w3.org/2000/svg" viewBox="-1 -1 11 11"><path stroke-linecap="round" stroke-width="1.5" d="m0 0 9,9 M0 9 9,0"></path></svg>',d=a(),p=c("section"),u=c("fieldset"),w=c("legend"),w.textContent="Login",m=a(),q=c("div"),q.innerHTML='<label for="username" class="svelte-1q3cvh4">Username</label> \n\t\t\t\t<input type="text" id="username" name="username" class="svelte-1q3cvh4"/>',f=a(),x=c("div"),g=c("label"),g.textContent="Password",b=a(),k=c("input"),L=a(),y=c("input"),M=a(),S=c("label"),S.textContent="Show password",H=a(),T=c("button"),T.textContent="Submit",j=a(),B=c("a"),B.textContent="Forgot password?",D=a(),A=c("p"),A.innerHTML='Don&#39;t have an account?\n\t\t\t<a href="/" class="svelte-1q3cvh4">Sign up</a>',F=a(),P=c("section"),document.title="Login",n(h,"href","/"),n(h,"id","close"),n(h,"aria-label","Close"),n(h,"class","svelte-1q3cvh4"),n(w,"class","svelte-1q3cvh4"),n(q,"class","svelte-1q3cvh4"),n(g,"for","password"),n(g,"class","svelte-1q3cvh4"),n(k,"type",C=s[0]?"text":"password"),n(k,"id","password"),n(k,"name","password"),n(k,"class","svelte-1q3cvh4"),n(y,"type","checkbox"),n(y,"id","show-password"),n(y,"name","show-password"),n(y,"class","svelte-1q3cvh4"),n(S,"class","inline svelte-1q3cvh4"),n(S,"for","show-password"),n(x,"class","svelte-1q3cvh4"),n(T,"class","svelte-1q3cvh4"),n(B,"href","/"),n(B,"class","svelte-1q3cvh4"),n(u,"class","svelte-1q3cvh4"),n(A,"class","svelte-1q3cvh4"),n(p,"id","login"),n(p,"class","svelte-1q3cvh4"),n(P,"id","background"),n(P,"class","svelte-1q3cvh4"),n(t,"class","svelte-1q3cvh4")},m(a,c){l(a,e,c),l(a,t,c),o(t,h),o(t,d),o(t,p),o(p,u),o(u,w),o(u,m),o(u,q),o(u,f),o(u,x),o(x,g),o(x,b),o(x,k),o(x,L),o(x,y),y.checked=s[0],o(x,M),o(x,S),o(u,H),o(u,T),o(u,j),o(u,B),o(p,D),o(p,A),o(t,F),o(t,P),U||(z=v(y,"change",s[1]),U=!0)},p(s,[e]){1&e&&C!==(C=s[0]?"text":"password")&&n(k,"type",C),1&e&&(y.checked=s[0])},i:r,o:r,d(s){s&&i(e),s&&i(t),U=!1,z()}}}function w(s,e,t){let a=!1;return h((()=>{p.update((s=>!1))})),d((()=>{p.update((s=>!0))})),[a,function(){a=this.checked,t(0,a)}]}export default class extends s{constructor(s){super(),e(this,s,w,u,t,{})}}
//# sourceMappingURL=Login-b6041564.js.map