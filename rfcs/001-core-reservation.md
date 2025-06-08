
# Feature
- Feature Name: core-reservation-service
- Start Date: 2024-06-08

## Summary
一种核心的预留服务，用户解决某一资源设定一段时间内使用期限的问题，我们用PostgreSql的EXCLUDE约束去对于给定的资源,在给定的时间短内只能进行一次预留操作

## Motivation
我们需要一个通用搞得解决方案来满足各种预定需求:1)日程安排预定; 2)酒店/房间预定;3)会议预定; 4)停车场预定等等;为满足这些需求而反复构建各种功能;
我们应当寻找一个通用的解决方案，供所有团队共同使用

## Guide-level explanation
![img.png](img.png)
## service interface
我们使用grpc构建服务接口，下面是proto 定义
```proto

enum ReservationStatus{
    UNKNOWN = 0;
    PENDING = 1;
    CONFIRM = 2;
    BLOCK = 3;
}

enum ReservationUpdateType{
    UNKNOWN = 0;
    CREATE = 1;
    UPDATE = 2;
    DELETE = 3;
}
message Reservation {
   string id = 1;
   string user_id = 2;
   ReservationStatus status = 3;
   //资源预定页面
   string resource_id = 4;
   google.protobuf.Timestamp start = 5;
   google.protobuf.Timestamp end = 6;

   //备注
   string note = 7;
}

message ReservationRequest {
    Reservation reservation = 1;
}

message ReservationResponse {
      Reservation reservation = 1;
}

message UpdateRequest {
    string note = 1;
}

message UpdateResponse {
     Reservation reservation = 1;
}


message ConfirmRequest {
    string id = 1;
}

message ConfirmResponse {
     Reservation reservation = 1;
}

message CancelRequest {
    string id = 1;
}

message CancelResponse {
     Reservation reservation = 1;
}

message GetRequest {
    string id = 1;
}

message GetResponse{
     Reservation reservation = 1;
}

message QueryRequest {
   string resource_id = 1;
   string user_id = 2;
   ReservationStatus status = 3;
   google.protobuf.Timestamp start = 4;
   google.protobuf.Timestamp end = 5;
}

message ListenRequest {}
message ListenResponse{
     int8 op = 1;
     Reservation reservation = 2;
}

service ReserviceService{
    rpc reserve(ReservationRequest) returns (ReservationResponse);
    rpc update(UpdateRequest) returns (UpdateResponse);
    rpc confirm(ConfirmRequest) returns (ConfirmResponse);
    rpc cancel(CancelRequest) returns (CancelResponse);
    rpc get(GetRequest) returns (GetResponse);
    rpc query(QueryRequest) returns (stream Reservation);
    //另一个系统监控新添加的预定信息
    rpc listen(ListenRequest) returns (stream Reservation);
}

```
## Data schema
使用postgresql的EXCLUDE;

```sql
CREATE SCHEMA rsvp;

CREATE TYPE rsvp.reservation_status AS ENUM
('unknown','pending','confirmed','blocked');
CREATE TYPE rsvp.reservation_update_type AS ENUM
('unknown','create','update','delete');
CREATE TABLE rsvp.reservations (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    user_id VARCHAR(64) NOT NULL,
    status reservation_status NOT NULL
    DEFAULT 'pending',
    resource_id VARCHAR(64) NOT NULL,
    start timestamptz NOT NULL,
    end timestamptz NOT NULL,
    timespan TSTZRANGE NOT NULL,
    note text,
    CONSTRAINT rsvp.reservation_pkey PRIMARY KEY (id),
    CONSTRAINT rsvp.reservation_conflict EXCLUDE
    USING gist (resource_id WITH=,timespan WITH &&)

);

CREATE INDEX reservation_resource_id_idx ON rsvp.reservations(resource_id);
CREATE INDEX reservation_user_id_idx ON rsvp.reservations(user_id);

-- 如果用户id为空,则查找该资源范围内所有的预定记录
CREATE OR REPLACE FUNCTION rsvp.query(uid text,rid text,during,TSTZRANGE)
RETURNS TABLE rsvp.reservations AS $$ $$ LANGUAGE plpgsql;


-- 预定变化队列
CREATE TABLE rsvp.reservation_changes (
    id SERIAL NOT NULL,
    reservation_id uuid NOT NULL,
    op rsvp.reservation_update_type NOT NULL,

);
-- trigger add/update/delete 预定
CREATE OR REPLACE FUNCTION rsvp.reservations_trigger() RETURNS TRIGGER AS
$$
BEGIN
     IF TG_OP='INSERT' THEN
        INSERT INTO rsvp.reservation_changes (reservation_id,op)
        VALUES(NEW.id,'create')
     ELSIF TG_OP='UPDATE' THEN
        IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_changes (reservation_id,op)
            VALUES(NEW.id,'update')
     END IF;

     ELSIF TG_OP='DELETE' THEN
           INSERT INTO rsvp.reservation_changes (reservation_id,op)
           VALUES(NEW.id,'create')
     END IF;
     NOTIFY reservations_update;
     RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER  reservations_trigger
    AFTER INSERT OR UPDATE OR DELETE ON rsvp.reservations
    FOR EACH RAW EXECUTE PROCEDURE rsvp.reservations_trigger();

```






Explain the proposal as if it was already included in the language and you were teaching it to another Rust programmer. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how Rust programmers should *think* about the feature, and how it should impact the way they use Rust. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing Rust programmers and new Rust programmers.
- Discuss how this impacts the ability to read, understand, and maintain Rust code. Code is read and modified far more often than written; will the proposed feature make code easier to maintain?

For implementation-oriented RFCs (e.g. for compiler internals), this section should focus on how compiler contributors should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.

## Reference-level explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

## Drawbacks

Why should we *not* do this?

## Rationale and alternatives


- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
- If this is a language proposal, could this be done in a library or macro instead? Does the proposed change make Rust code easier or harder to read, understand, and maintain?

## Prior art


Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For language, library, cargo, tools, and compiler proposals: Does this feature exist in other programming languages and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.
Please also take into consideration that rust sometimes intentionally diverges from common language features.

## Unresolved questions


- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

## Future possibilities


Think about what the natural extension and evolution of your proposal would
be and how it would affect the language and project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project and language in your proposal.
Also consider how this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
The section merely provides additional information.
