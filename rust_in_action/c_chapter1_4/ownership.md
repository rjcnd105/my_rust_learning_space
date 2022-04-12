두려움 없는 동시성 프로그래밍을 하자.

**keyword:** _ownership, owners, lifetime, borrow, borrow checker, moved_

## Ownership(소유권)

- rust의 모든 값은 Owned(소유됨)
- Owner(소유자)는 value의 lifetime이 다하면 정리한다
- 유형에 대한 사용자 정의 소멸를 제공하려면 Drop을 impl하라
- Owner(소유자)는 자신의 값에 대해 특별한 엑세스 권한이 없으며, 다른 이의 침입을 막는 권한도 없다 (이게 뭔솔... 소유자는 별 권한이 없으면서, 시스템적으로 정해져 있다는 것인가?)
- Owner(소유자)는 다른 부분이 자신의 값에 엑세스 하는 것을 막거나, 데이터 도난을 막는다

## moved(소유자 이전)

moved(소유자 이전, ownership moved)시 기존 Owners로 접근 불가능

### 이전 방법

- 방법1. 다른 변수에 할당(variable binding)
- 방법2. 인수 또는 반환 값으로 함수를 통해 전달

### 이전하지 않는 방법

- 방법1. ref참조를 넘김 ```&variable``` (완전한 소유권이 필요하지 않은 경우)
- 방법2. 함수에서 다시 리턴시 소유권을 원래 변수로 되돌림 ```fn return_to(v: MyData) -> MyData```
- 방법3. 값 복제 사용

## Lifetime(수명)

- lifetime은 해당 값에 엑세스할 수 있는 기간
- lifetime이 최대한 적게, 더 개별적이고 일시적인 수명을 가지게 하는게 좋다

## Borrow

- 소유자에게 반환할 필요 없이 해당 값에 엑세스할 수 있어짐
- 단일 소유자를 가지고 있지만, 해당 값에 대한 엑세스를 공유할 수 있음을 시사
- Borrowed하지 않은 값에 대해서는 rebinding이(같은 변수에 다시 값을 바인딩 시키는 것) 합법이다 
- read-write borrow는 오직 1개만 존재 가능하다
- borrow checker에 의해 거부당하면, 값을 Clone, Copy하는 전략이 좋은 방법이 될 수 있다.

## 전략
1. 래퍼 유형(ex: ```std:rc::Rc<T>``` (여기서 T는 참조 카운트 값))을 통해 공유 소유권을 제공한다. 공유 소유권을 제공하면 모든 소유자가 제거될 때까지 ```T```가 메모리에서 제거되지 않는다. Rc::clone()을 호출하면 내부 카운트를 증가시킨다. 카운트가 0일때 원래 인스턴스가 해제된다.
2. 프로그래머가 가비지 컬렉션을 할 수 있도록 제공